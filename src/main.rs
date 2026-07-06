mod action;
mod grid;
mod hints;
mod overlay;

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "macos")]
fn main() {
    if !macos::is_trusted() {
        eprintln!(
            "typetap needs Accessibility permission.\n\
             System Settings -> Privacy & Security -> Accessibility -> enable your terminal,\n\
             then run again."
        );
        std::process::exit(1);
    }

    // typetap runs as a background daemon: press the global hotkey (⇧⌘Space)
    // from any app and the overlay appears over whatever is on screen.
    //
    //   `cargo run`         -> element mode: hint every clickable AX element.
    //   `cargo run -- grid` -> grid mode: label a whole-screen grid, so you can
    //                          click anywhere even when the app exposes no AX
    //                          elements (games, canvas apps, a plain terminal).
    let grid_mode = std::env::args().nth(1).as_deref() == Some("grid");

    eprintln!(
        "typetap ready ({} mode). Press \u{21e7}\u{2318}Space to show hints; Esc to cancel; Ctrl-C to quit.",
        if grid_mode { "grid" } else { "element" }
    );

    // Called on each hotkey press to build the hints for the current screen.
    // The overlay covers a single display (never the union — that can exceed the
    // GPU surface limit and abort). Grid mode covers the display under the
    // cursor; element mode covers the display holding the target elements.
    let gather = move || -> Option<overlay::Scene> {
        if grid_mode {
            let cursor = macos::cursor_point().unwrap_or((0.0, 0.0));
            let (origin, size) = macos::display_containing(cursor);
            let (cols, rows) = grid::dims(size, 80.0);
            let hints = grid::cells(origin, size, cols, rows)
                .into_iter()
                .map(|c| overlay::Hint {
                    label: c.label,
                    x: c.x,
                    y: c.y,
                })
                .collect();
            Some(overlay::Scene {
                origin,
                size,
                hints,
                grid: Some(overlay::Grid { cols, rows }),
            })
        } else {
            let elements = match macos::clickable_elements() {
                Ok(e) => e,
                Err(e) => {
                    eprintln!("could not read elements: {e}");
                    return None;
                }
            };
            if elements.is_empty() {
                eprintln!("(no clickable elements in the front window — try grid mode)");
                return None;
            }

            // All elements of one window share a display; cover that one and keep
            // only the elements on it so none render at the wrong offset.
            let (origin, size) = macos::display_containing((elements[0].x, elements[0].y));
            let on_display: Vec<macos::Element> = elements
                .into_iter()
                .filter(|e| {
                    e.x >= origin.0
                        && e.x < origin.0 + size.0
                        && e.y >= origin.1
                        && e.y < origin.1 + size.1
                })
                .collect();
            if on_display.is_empty() {
                return None;
            }

            let priorities: Vec<i32> = on_display.iter().map(|e| e.priority).collect();
            let labels = hints::assign(&priorities);
            let hints = on_display
                .iter()
                .zip(labels)
                .map(|(el, label)| overlay::Hint {
                    label,
                    x: el.x,
                    y: el.y,
                })
                .collect();
            Some(overlay::Scene {
                origin,
                size,
                hints,
                grid: None,
            })
        }
    };

    // Runs until the process is killed.
    overlay::run(gather);
}

#[cfg(not(target_os = "macos"))]
fn main() {
    eprintln!("typetap currently only supports macOS.");
}
