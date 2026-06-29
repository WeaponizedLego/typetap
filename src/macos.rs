//! macOS Accessibility backend: find clickable elements in the focused window.
//!
//! This is the only OS-specific file so far. It produces a plain `Vec<Element>`
//! (role + screen position + priority); everything downstream (hint assignment,
//! later the overlay) stays OS-independent.

// The `cocoa`/`objc` crates emit deprecation + macro-cfg noise we don't care
// about. Quiet them so build output stays readable.
#![allow(deprecated, unexpected_cfgs)]

use std::cell::RefCell;
use std::ffi::c_void;

use accessibility::{AXAttribute, AXUIElement, TreeVisitor, TreeWalker, TreeWalkerFlow};
use accessibility_sys::{
    kAXValueTypeCGPoint, kAXValueTypeCGSize, AXIsProcessTrusted, AXValueGetValue, AXValueRef,
};
use cocoa::base::{id, nil};
use core_foundation::base::{CFType, TCFType};
use core_foundation::string::CFString;
use core_graphics::display::CGDisplay;
use core_graphics::geometry::{CGPoint, CGSize};
use objc::{class, msg_send, sel, sel_impl};

/// One clickable thing on screen.
#[derive(Debug, Clone)]
pub struct Element {
    /// Kept for debugging / future per-role styling, not currently read.
    #[allow(dead_code)]
    pub role: String,
    /// Center point in screen coordinates — where a click would land.
    pub x: f64,
    pub y: f64,
    /// Click-intent score; higher = more likely a real click target.
    pub priority: i32,
}

/// Has the user granted this process Accessibility permission?
/// Without it macOS hands back empty trees for every other app.
pub fn is_trusted() -> bool {
    unsafe { AXIsProcessTrusted() }
}

/// Map an AX role to a click-intent priority. 0 = not a click target (skip).
///
/// ponytail: role-only heuristic. Good enough — upgrade to also honoring the
/// AXPress action per element only if real windows surface clickable things
/// this misses.
fn role_priority(role: &str) -> i32 {
    match role {
        "AXButton" | "AXPopUpButton" | "AXMenuButton" | "AXMenuItem" | "AXMenuBarItem"
        | "AXLink" => 100,
        "AXCheckBox" | "AXRadioButton" | "AXDisclosureTriangle" | "AXColorWell" => 90,
        "AXComboBox" | "AXSlider" | "AXIncrementor" | "AXTab" => 80,
        "AXTextField" | "AXTextArea" | "AXSearchField" | "AXDateField" | "AXTimeField" => 70,
        "AXCell" | "AXRow" => 40,
        "AXImage" => 20,
        "AXStaticText" => 10,
        _ => 0,
    }
}

/// Read an attribute that comes back as an AXValue point/size into (x, y).
fn read_axvalue_pair(el: &AXUIElement, attr: &'static str, kind: u32) -> Option<(f64, f64)> {
    let attribute: AXAttribute<core_foundation::base::CFType> =
        AXAttribute::new(&CFString::from_static_string(attr));
    let value = el.attribute(&attribute).ok()?;
    unsafe {
        let raw = value.as_CFTypeRef() as AXValueRef;
        if kind == kAXValueTypeCGPoint {
            let mut p = CGPoint { x: 0.0, y: 0.0 };
            if AXValueGetValue(raw, kind, &mut p as *mut _ as *mut c_void) {
                return Some((p.x, p.y));
            }
        } else {
            let mut s = CGSize {
                width: 0.0,
                height: 0.0,
            };
            if AXValueGetValue(raw, kind, &mut s as *mut _ as *mut c_void) {
                return Some((s.width, s.height));
            }
        }
    }
    None
}

/// Collects clickable elements while walking the AX tree.
struct Collector {
    out: RefCell<Vec<Element>>,
}

impl TreeVisitor for Collector {
    fn enter_element(&self, el: &AXUIElement) -> TreeWalkerFlow {
        if let Ok(role) = el.attribute(&AXAttribute::role()) {
            let role = role.to_string();
            let priority = role_priority(&role);
            if priority > 0 {
                if let (Some((px, py)), Some((w, h))) = (
                    read_axvalue_pair(el, "AXPosition", kAXValueTypeCGPoint),
                    read_axvalue_pair(el, "AXSize", kAXValueTypeCGSize),
                ) {
                    // Skip zero-area / offscreen junk.
                    if w >= 1.0 && h >= 1.0 {
                        self.out.borrow_mut().push(Element {
                            role,
                            x: px + w / 2.0,
                            y: py + h / 2.0,
                            priority,
                        });
                    }
                }
            }
        }
        TreeWalkerFlow::Continue
    }

    fn exit_element(&self, _el: &AXUIElement) {}
}

/// Bounding rectangle covering all displays, as (origin, size) in screen
/// points. The overlay window is sized to this so hints on any monitor land in
/// the right place (secondary monitors can sit at negative coordinates).
pub fn screen_bounds() -> ((f64, f64), (f64, f64)) {
    let ids = CGDisplay::active_displays().unwrap_or_default();
    if ids.is_empty() {
        let b = CGDisplay::main().bounds();
        return ((b.origin.x, b.origin.y), (b.size.width, b.size.height));
    }
    let (mut min_x, mut min_y, mut max_x, mut max_y) = (f64::MAX, f64::MAX, f64::MIN, f64::MIN);
    for id in ids {
        let b = CGDisplay::new(id).bounds();
        min_x = min_x.min(b.origin.x);
        min_y = min_y.min(b.origin.y);
        max_x = max_x.max(b.origin.x + b.size.width);
        max_y = max_y.max(b.origin.y + b.size.height);
    }
    ((min_x, min_y), (max_x - min_x, max_y - min_y))
}

/// PID of the app the user is currently looking at, via AppKit's NSWorkspace.
/// More reliable than the system-wide AXFocusedApplication attribute.
fn frontmost_pid() -> Option<i32> {
    unsafe {
        let workspace: id = msg_send![class!(NSWorkspace), sharedWorkspace];
        if workspace == nil {
            return None;
        }
        let app: id = msg_send![workspace, frontmostApplication];
        if app == nil {
            return None;
        }
        let pid: i32 = msg_send![app, processIdentifier];
        Some(pid)
    }
}

/// Enumerate clickable elements in the currently focused window.
pub fn clickable_elements() -> Result<Vec<Element>, String> {
    let app = match frontmost_pid() {
        // Preferred: the app the user is actually looking at.
        Some(pid) => AXUIElement::application(pid),
        // Fallback: ask the system for whatever currently has focus.
        None => {
            let system = AXUIElement::system_wide();
            let attr: AXAttribute<CFType> =
                AXAttribute::new(&CFString::from_static_string("AXFocusedApplication"));
            system
                .attribute(&attr)
                .map_err(|e| format!("no frontmost or focused application: {e}"))?
                .downcast_into::<AXUIElement>()
                .ok_or("focused application is not a UI element")?
        }
    };

    let window = app
        .attribute(&AXAttribute::focused_window())
        .or_else(|_| app.attribute(&AXAttribute::main_window()))
        .map_err(|e| format!("no focused window: {e}"))?;

    let collector = Collector {
        out: RefCell::new(Vec::new()),
    };
    TreeWalker::new().walk(&window, &collector);
    Ok(collector.out.into_inner())
}
