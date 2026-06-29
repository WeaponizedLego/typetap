//! Transparent, always-on-top overlay that draws hint chips on top of the real
//! screen elements and reads the user's keystrokes. Built on egui, so it
//! carries over to the Windows version.
//!
//! Coordinates everywhere are screen points (top-left origin), the same space
//! the Accessibility API and the clicker use.

use std::sync::{Arc, Mutex};

use eframe::egui;

use crate::action::Action;

/// A label to draw at a screen position.
pub struct Hint {
    pub label: String,
    pub x: f64,
    pub y: f64,
}

/// What the user picked.
pub struct Outcome {
    pub action: Action,
    pub x: f64,
    pub y: f64,
}

/// Show the overlay covering the rectangle `origin`..`origin+size` (screen
/// points). Blocks until the user picks a hint or presses Escape. Returns the
/// chosen action+target, or None if cancelled.
pub fn show(origin: (f64, f64), size: (f64, f64), hints: Vec<Hint>) -> Option<Outcome> {
    let result: Arc<Mutex<Option<Outcome>>> = Arc::new(Mutex::new(None));

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_position(egui::pos2(origin.0 as f32, origin.1 as f32))
            .with_inner_size(egui::vec2(size.0 as f32, size.1 as f32))
            .with_decorations(false)
            .with_transparent(true)
            .with_always_on_top()
            .with_active(true)
            .with_taskbar(false),
        ..Default::default()
    };

    let app = OverlayApp {
        hints,
        origin,
        typed: String::new(),
        result: result.clone(),
    };

    // If the window fails to open we just return None rather than crashing.
    let _ = eframe::run_native(
        "typetap",
        options,
        Box::new(move |_cc| Ok(Box::new(app))),
    );

    Arc::try_unwrap(result)
        .ok()
        .and_then(|m| m.into_inner().ok())
        .flatten()
}

struct OverlayApp {
    hints: Vec<Hint>,
    origin: (f64, f64),
    typed: String,
    result: Arc<Mutex<Option<Outcome>>>,
}

impl eframe::App for OverlayApp {
    // Fully transparent window; we only paint the chips + a faint dim.
    fn clear_color(&self, _v: &egui::Visuals) -> [f32; 4] {
        [0.0, 0.0, 0.0, 0.0]
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();

        // --- read keystrokes -------------------------------------------------
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
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            return;
        }

        // --- exact match fires the action ------------------------------------
        // Labels are prefix-free, so a full match is unambiguous.
        if let Some(hint) = self.hints.iter().find(|h| h.label == self.typed) {
            // Modifiers choose the action: Shift = right-click, Option = move.
            let action = if modifiers.shift {
                Action::RightClick
            } else if modifiers.alt {
                Action::Move
            } else {
                Action::LeftClick
            };
            *self.result.lock().unwrap() = Some(Outcome {
                action,
                x: hint.x,
                y: hint.y,
            });
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            return;
        }

        // --- draw ------------------------------------------------------------
        let painter = ui.painter();
        // Faint dim so chips read over busy backgrounds.
        painter.rect_filled(
            ui.max_rect(),
            0.0,
            egui::Color32::from_rgba_unmultiplied(0, 0, 0, 60),
        );

        let font = egui::FontId::monospace(15.0);
        for hint in &self.hints {
            // Hide hints that can't match what's typed so far.
            if !hint.label.starts_with(&self.typed) {
                continue;
            }
            let pos = egui::pos2(
                (hint.x - self.origin.0) as f32,
                (hint.y - self.origin.1) as f32,
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

        // Repaint promptly so typing feels responsive.
        ctx.request_repaint();
    }
}
