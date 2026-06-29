mod action;
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

    let elements = match macos::clickable_elements() {
        Ok(e) => e,
        Err(e) => {
            eprintln!("could not read elements: {e}");
            std::process::exit(1);
        }
    };

    if elements.is_empty() {
        eprintln!("No clickable elements found in the focused window.");
        return;
    }

    let priorities: Vec<i32> = elements.iter().map(|e| e.priority).collect();
    let labels = hints::assign(&priorities);

    let hints: Vec<overlay::Hint> = elements
        .iter()
        .zip(labels)
        .map(|(el, label)| overlay::Hint {
            label,
            x: el.x,
            y: el.y,
        })
        .collect();

    let (origin, size) = macos::screen_bounds();

    // Show the overlay; once it closes, perform whatever the user picked.
    if let Some(out) = overlay::show(origin, size, hints) {
        if let Err(e) = action::perform(out.action, out.x, out.y) {
            eprintln!("action failed: {e}");
            std::process::exit(1);
        }
    }
}

#[cfg(not(target_os = "macos"))]
fn main() {
    eprintln!("typetap currently only supports macOS.");
}
