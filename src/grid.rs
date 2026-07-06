//! Whole-screen grid mode. When the frontmost app exposes no clickable
//! Accessibility elements — games, canvas/Electron apps, remote desktops, or a
//! plain terminal — element-hint mode has nothing to label. Grid mode falls
//! back to overlaying a labeled grid across the whole screen so the keyboard
//! can still aim a click at any point.
//!
//! Pure geometry + labeling, no OS code — portable like `hints`, so the future
//! Windows backend reuses it unchanged.

use crate::hints::EASE_ORDER;

/// One grid cell: the hint the user types and the screen-point at its center
/// (where the click lands).
pub struct Cell {
    pub label: String,
    pub x: f64,
    pub y: f64,
}

/// Pick a `(cols, rows)` grid for a screen of `size` points, aiming for roughly
/// square cells about `target` points on a side. Clamped to `1..=26` on each
/// axis so every cell keeps a two-key `row`+`col` label (26×26 = 676 cells max).
///
/// ponytail: no zoom/refine step, so on very large (4K+) displays cells stay
/// coarse. Add a "type a cell, then a finer sub-grid" pass only if real use
/// shows the coarse grid misses small targets.
pub fn dims(size: (f64, f64), target: f64) -> (usize, usize) {
    let axis = |len: f64| ((len / target).round() as i64).clamp(1, EASE_ORDER.len() as i64) as usize;
    (axis(size.0), axis(size.1))
}

/// Build the `cols`×`rows` grid covering `origin`..`origin + size` (screen
/// points). Cells are labeled row-major: the first key selects the row, the
/// second the column, both in typing-ease order — so the easiest chords land in
/// the top-left, where windows usually open.
pub fn cells(origin: (f64, f64), size: (f64, f64), cols: usize, rows: usize) -> Vec<Cell> {
    let cols = cols.clamp(1, EASE_ORDER.len());
    let rows = rows.clamp(1, EASE_ORDER.len());
    let cell_w = size.0 / cols as f64;
    let cell_h = size.1 / rows as f64;

    let mut out = Vec::with_capacity(cols * rows);
    for r in 0..rows {
        for c in 0..cols {
            out.push(Cell {
                label: format!("{}{}", EASE_ORDER[r], EASE_ORDER[c]),
                x: origin.0 + (c as f64 + 0.5) * cell_w,
                y: origin.1 + (r as f64 + 0.5) * cell_h,
            });
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dims_round_and_clamp() {
        // Rounds to the nearest whole cell count.
        assert_eq!(dims((1512.0, 982.0), 80.0), (19, 12));
        // Never below 1×1, even for a tiny screen.
        assert_eq!(dims((10.0, 10.0), 80.0), (1, 1));
        // Never above the 26-key alphabet, even on a huge display.
        assert_eq!(dims((8000.0, 8000.0), 80.0), (26, 26));
    }

    #[test]
    fn cell_count_matches_dims() {
        let cells = cells((0.0, 0.0), (800.0, 600.0), 4, 3);
        assert_eq!(cells.len(), 12);
    }

    #[test]
    fn labels_are_unique_and_two_keys() {
        let cells = cells((0.0, 0.0), (800.0, 600.0), 5, 4);
        for cell in &cells {
            assert_eq!(cell.label.len(), 2, "grid labels are always two keys");
        }
        let uniq: std::collections::HashSet<_> = cells.iter().map(|c| &c.label).collect();
        assert_eq!(uniq.len(), cells.len(), "labels must be unique");
    }

    #[test]
    fn top_left_cell_gets_the_easiest_chord() {
        let cells = cells((0.0, 0.0), (800.0, 600.0), 4, 3);
        // EASE_ORDER[0] is 'f' → the top-left cell is "ff".
        assert_eq!(cells[0].label, "ff");
    }

    #[test]
    fn centers_sit_in_the_middle_of_each_cell() {
        // 2×1 grid over an 800×600 screen: cells are 400 wide, centers at 200 & 600.
        let cells = cells((0.0, 0.0), (800.0, 600.0), 2, 1);
        assert_eq!((cells[0].x, cells[0].y), (200.0, 300.0));
        assert_eq!((cells[1].x, cells[1].y), (600.0, 300.0));
    }

    #[test]
    fn origin_offsets_the_whole_grid() {
        // Secondary monitors can sit at negative screen coordinates.
        let cells = cells((-1440.0, 0.0), (1440.0, 900.0), 2, 1);
        assert_eq!(cells[0].x, -1080.0); // -1440 + 0.5 * 720
    }
}
