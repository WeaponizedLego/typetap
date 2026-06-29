//! Performing the actual mouse action. Cross-platform via enigo (works on
//! Windows too), so this file has no OS-specific code.

use enigo::{Button, Coordinate, Direction, Enigo, Mouse, Settings};

/// What to do once the user picks an element.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    LeftClick,
    RightClick,
    /// Move the pointer there without clicking.
    Move,
}

/// Move the pointer to (x, y) in screen coordinates and perform `action`.
/// Coordinates are AX points (top-left origin), which match the global
/// coordinate space enigo uses on macOS — including negative values on
/// secondary monitors.
pub fn perform(action: Action, x: f64, y: f64) -> Result<(), String> {
    let mut enigo = Enigo::new(&Settings::default()).map_err(|e| format!("{e:?}"))?;
    enigo
        .move_mouse(x as i32, y as i32, Coordinate::Abs)
        .map_err(|e| format!("{e:?}"))?;
    match action {
        Action::Move => Ok(()),
        Action::LeftClick => enigo
            .button(Button::Left, Direction::Click)
            .map_err(|e| format!("{e:?}")),
        Action::RightClick => enigo
            .button(Button::Right, Direction::Click)
            .map_err(|e| format!("{e:?}")),
    }
}
