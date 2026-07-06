//! Persistent, transparent overlay driven by a global hotkey.
//!
//! winit allows only one event loop per process, so we can't open a fresh
//! window on each hotkey press. Instead the app runs for the whole process
//! lifetime with a single window. A global-hotkey handler wakes the (otherwise
//! sleeping) event loop; on trigger we ask the caller to rebuild the hints for
//! whatever is on screen right now, position the window over the target
//! display, and read keystrokes. Picking a hint clicks the app beneath, ready
//! for the next press.
//!
//! IMPORTANT: after its first reveal the window is *never* ordered out again.
//! Once you hide a window with `Visible(false)`, macOS (and Windows 11) stop
//! delivering it redraw events, so eframe never calls `App::update` again and
//! the hotkey handler's `request_repaint()` is silently dropped — the overlay
//! would fire exactly once per launch and then go dead (egui #5112, #5229). So
//! we "hide" by painting nothing: the window is fully transparent and
//! mouse-passthrough, so when idle it's invisible and inert while still on
//! screen. To hand the keyboard back to the app beneath we deactivate our own
//! app (`relinquish_focus`) rather than hiding the window.
//!
//! egui-based and OS-agnostic (the caller injects the screen-scanning via
//! `gather`), so it carries over to the Windows version.
//!
//! Coordinates everywhere are screen points (top-left origin), the same space
//! the Accessibility API and the clicker use.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use eframe::egui;
use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState,
};

use crate::action::{self, Action};

/// A label to draw at a screen position.
pub struct Hint {
    pub label: String,
    pub x: f64,
    pub y: f64,
}

/// Grid-mode decoration: draw faint `cols`×`rows` cell borders behind the
/// chips so the user can see the grid the hints belong to. `None` for
/// element-hint mode, which paints chips directly on real UI.
pub struct Grid {
    pub cols: usize,
    pub rows: usize,
}

/// One overlay activation: which display to cover (screen points) and what to
/// paint on it. The caller builds this fresh on every hotkey press.
pub struct Scene {
    pub origin: (f64, f64),
    pub size: (f64, f64),
    pub hints: Vec<Hint>,
    pub grid: Option<Grid>,
}

/// What the overlay is doing right now.
enum Mode {
    /// Hidden, waiting for the hotkey.
    Idle,
    /// Shown, reading keystrokes against `scene`.
    Active(Scene),
    /// Hidden again and counting down a couple of frames before clicking, so
    /// the window has actually disappeared and the click lands on the app
    /// beneath us rather than on our own overlay.
    Firing {
        action: Action,
        x: f64,
        y: f64,
        ticks: u8,
    },
}

/// Run the overlay until the process exits. `gather` is called on each hotkey
/// press to build the scene for the current screen; returning `None` (e.g. no
/// clickable elements) just leaves the overlay hidden.
pub fn run<F: FnMut() -> Option<Scene> + 'static>(gather: F) {
    let manager = GlobalHotKeyManager::new().expect("create global hotkey manager");

    // ⇧⌘Space on macOS, Shift+Alt+Space elsewhere (Windows/Linux).
    #[cfg(target_os = "macos")]
    let hotkey = HotKey::new(Some(Modifiers::SHIFT | Modifiers::SUPER), Code::Space);
    #[cfg(not(target_os = "macos"))]
    let hotkey = HotKey::new(Some(Modifiers::SHIFT | Modifiers::ALT), Code::Space);
    manager.register(hotkey).expect("register global hotkey");

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(egui::vec2(800.0, 600.0))
            .with_decorations(false)
            .with_transparent(true)
            .with_always_on_top()
            // The overlay only ever reads the keyboard; letting the mouse fall
            // through means our synthetic click can't land on our own window.
            .with_mouse_passthrough(true)
            // Start hidden and inactive so launching doesn't flash a dim screen
            // or steal focus — the hotkey reveals it on demand.
            .with_visible(false)
            .with_active(false)
            .with_taskbar(false),
        ..Default::default()
    };

    // Set by the hotkey handler (below) to ask the idle loop to show itself.
    let pending = Arc::new(AtomicBool::new(false));

    // If the window fails to open we just fall through and exit.
    let _ = eframe::run_native(
        "typetap",
        options,
        Box::new(move |cc| {
            // A hidden window gets no input events, so a repaint timer can't be
            // trusted to keep polling. Instead, wake the event loop from the
            // hotkey handler itself — it runs on the app's run loop regardless
            // of our window state, so this fires reliably every time.
            let ctx = cc.egui_ctx.clone();
            let flag = pending.clone();
            GlobalHotKeyEvent::set_event_handler(Some(move |event: GlobalHotKeyEvent| {
                if event.state == HotKeyState::Pressed {
                    flag.store(true, Ordering::SeqCst);
                    ctx.request_repaint();
                }
            }));

            Ok(Box::new(OverlayApp {
                gather: Box::new(gather),
                _manager: manager,
                pending,
                mode: Mode::Idle,
                typed: String::new(),
            }))
        }),
    );
}

struct OverlayApp {
    gather: Box<dyn FnMut() -> Option<Scene>>,
    // Kept alive for the whole run: dropping the manager unregisters the hotkey.
    _manager: GlobalHotKeyManager,
    pending: Arc<AtomicBool>,
    mode: Mode,
    typed: String,
}

impl OverlayApp {
    /// Reset to idle without acting. We don't order the window out (that would
    /// stop the event loop from ever waking again — see the module docs); the
    /// idle frame paints nothing, so the transparent window just goes invisible.
    fn dismiss(&mut self, _ctx: &egui::Context) {
        relinquish_focus();
        self.mode = Mode::Idle;
        self.typed.clear();
        // Ignore any press that arrived while we were shown.
        self.pending.store(false, Ordering::SeqCst);
    }
}

/// Hand keyboard focus back to the app beneath us. We grab focus when the
/// overlay activates (to read keystrokes), so on the way out we have to give it
/// back — and we can't do that by hiding the window (see the module docs).
/// Deactivating our own app returns activation to the previously frontmost app.
#[cfg_attr(target_os = "macos", allow(deprecated))] // cocoa::base::{id, nil}
fn relinquish_focus() {
    #[cfg(target_os = "macos")]
    unsafe {
        use cocoa::base::{id, nil};
        use objc::{class, msg_send, sel, sel_impl};
        let app: id = msg_send![class!(NSApplication), sharedApplication];
        if app != nil {
            let _: () = msg_send![app, deactivate];
        }
    }
}

impl eframe::App for OverlayApp {
    // Fully transparent window; we only paint the chips + a faint dim.
    fn clear_color(&self, _v: &egui::Visuals) -> [f32; 4] {
        [0.0, 0.0, 0.0, 0.0]
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();

        // --- idle: hidden, woken by the hotkey handler -----------------------
        if matches!(self.mode, Mode::Idle) {
            if self.pending.swap(false, Ordering::SeqCst) {
                if let Some(scene) = (self.gather)() {
                    // Move + size the window over the target display, then reveal.
                    ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(egui::pos2(
                        scene.origin.0 as f32,
                        scene.origin.1 as f32,
                    )));
                    ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(
                        scene.size.0 as f32,
                        scene.size.1 as f32,
                    )));
                    ctx.send_viewport_cmd(egui::ViewportCommand::Visible(true));
                    ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
                    self.typed.clear();
                    self.mode = Mode::Active(scene);
                    // Draw the chips on the very next frame.
                    ctx.request_repaint();
                }
            }
            // Otherwise nothing to do; sleep until the handler wakes us again.
            return;
        }

        // --- firing: window is hiding; click once it's gone ------------------
        if let &Mode::Firing { action, x, y, ticks } = &self.mode {
            if ticks > 0 {
                self.mode = Mode::Firing {
                    action,
                    x,
                    y,
                    ticks: ticks - 1,
                };
                ctx.request_repaint();
            } else {
                self.mode = Mode::Idle;
                if let Err(e) = action::perform(action, x, y) {
                    eprintln!("action failed: {e}");
                }
            }
            return;
        }

        // --- active: read keystrokes -----------------------------------------
        let mut escape = false;
        let modifiers = ctx.input(|i| {
            for event in &i.events {
                match event {
                    egui::Event::Text(t) => {
                        for c in t.chars().filter(|c| c.is_ascii_alphabetic()) {
                            self.typed.push(c.to_ascii_lowercase());
                        }
                    }
                    egui::Event::Key {
                        key: egui::Key::Backspace,
                        pressed: true,
                        ..
                    } => {
                        self.typed.pop();
                    }
                    egui::Event::Key {
                        key: egui::Key::Escape,
                        pressed: true,
                        ..
                    } => escape = true,
                    _ => {}
                }
            }
            i.modifiers
        });

        if escape {
            self.dismiss(&ctx);
            return;
        }

        // --- exact match fires the action ------------------------------------
        // Labels are prefix-free, so a full match is unambiguous.
        let hit = if let Mode::Active(scene) = &self.mode {
            scene
                .hints
                .iter()
                .find(|h| h.label == self.typed)
                .map(|h| (h.x, h.y))
        } else {
            None
        };
        if let Some((x, y)) = hit {
            // Modifiers choose the action: Shift = right-click, Option = move.
            let action = if modifiers.shift {
                Action::RightClick
            } else if modifiers.alt {
                Action::Move
            } else {
                Action::LeftClick
            };
            // Go invisible now (the Firing frame paints nothing) and hand focus
            // back; click a couple frames later so the overlay has stopped
            // painting. The click reaches the app beneath regardless because the
            // window is mouse-passthrough.
            relinquish_focus();
            self.typed.clear();
            self.mode = Mode::Firing {
                action,
                x,
                y,
                ticks: 2,
            };
            ctx.request_repaint();
            return;
        }

        // --- draw ------------------------------------------------------------
        let Mode::Active(scene) = &self.mode else {
            return;
        };
        let painter = ui.painter();
        let rect = ui.max_rect();
        // Faint dim so chips read over busy backgrounds.
        painter.rect_filled(
            rect,
            0.0,
            egui::Color32::from_rgba_unmultiplied(0, 0, 0, 60),
        );

        // Grid mode: outline the cells so the hints read as a grid.
        if let Some(grid) = &scene.grid {
            let stroke = egui::Stroke::new(1.0, egui::Color32::from_rgba_unmultiplied(255, 214, 10, 60));
            for c in 1..grid.cols {
                let x = rect.left() + rect.width() * c as f32 / grid.cols as f32;
                painter.line_segment([egui::pos2(x, rect.top()), egui::pos2(x, rect.bottom())], stroke);
            }
            for r in 1..grid.rows {
                let y = rect.top() + rect.height() * r as f32 / grid.rows as f32;
                painter.line_segment([egui::pos2(rect.left(), y), egui::pos2(rect.right(), y)], stroke);
            }
        }

        let font = egui::FontId::monospace(15.0);
        for hint in &scene.hints {
            // Hide hints that can't match what's typed so far.
            if !hint.label.starts_with(&self.typed) {
                continue;
            }
            let pos = egui::pos2(
                (hint.x - scene.origin.0) as f32,
                (hint.y - scene.origin.1) as f32,
            );
            let chip = egui::Rect::from_center_size(
                pos,
                egui::vec2(14.0 * hint.label.len().max(1) as f32 + 8.0, 20.0),
            );
            painter.rect_filled(chip, 4.0, egui::Color32::from_rgb(255, 214, 10));

            // The portion already typed is dimmed, the next key stands out.
            let typed_len = self.typed.len().min(hint.label.len());
            let remaining = &hint.label[typed_len..];
            painter.text(
                chip.center(),
                egui::Align2::CENTER_CENTER,
                remaining,
                font.clone(),
                egui::Color32::BLACK,
            );
        }

        // No unconditional repaint: keystrokes arrive as input events (each one
        // triggers a redraw), so the shown overlay updates without spinning.
    }
}
