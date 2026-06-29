//! Pure hint-label assignment. No OS deps — portable, testable.
//!
//! Goal: easiest keystrokes land on the elements most likely to be clicked.
//! The macOS/Windows layers just hand us a per-element priority (higher = more
//! click-intent, e.g. a button beats static text); all the cleverness is here.

/// Keys in typing-ease order: home-row index/middle fingers first, awkward
/// pinky/bottom-row keys last. Easiest keys become single-char hints on the
/// highest-priority elements; the hardest keys get demoted to multi-char prefixes.
pub const EASE_ORDER: [char; 26] = [
    'f', 'j', 'd', 'k', 's', 'l', 'a', 'g', 'h', 'r', 'u', 'e', 'i', 'w', 'o',
    'v', 'n', 'c', 'm', 't', 'y', 'b', 'p', 'q', 'x', 'z',
];

/// Assign a hint label to each element. `priorities[i]` is element i's
/// click-intent score. Returns labels parallel to the input (element i -> ret[i]).
///
/// Labels are prefix-free (no label is a prefix of another, so typing is
/// unambiguous) and the easiest labels go to the highest-priority elements.
pub fn assign(priorities: &[i32]) -> Vec<String> {
    assign_with(priorities, &EASE_ORDER)
}

fn assign_with(priorities: &[i32], alphabet: &[char]) -> Vec<String> {
    let n = priorities.len();
    if n == 0 {
        return Vec::new();
    }
    assert!(alphabet.len() >= 2, "need at least 2 keys to make hints");

    let labels = easiest_labels(n, alphabet);

    // Elements, hardest-intent broken by original order (stable) so reading
    // order wins ties. Best label -> highest priority element.
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|&a, &b| priorities[b].cmp(&priorities[a]));

    let mut out = vec![String::new(); n];
    for (label, &idx) in labels.into_iter().zip(order.iter()) {
        out[idx] = label;
    }
    out
}

/// The `n` easiest prefix-free labels, easiest first.
///
/// Start with single chars; when we need more, expand the *hardest* current
/// label into its children (label + each key). Removing the parent keeps the
/// set prefix-free, and demoting the hardest label means easy single keys stay
/// reserved for top-priority elements.
///
/// ponytail: greedy "expand the worst" — good, not provably optimal ergonomics.
/// Swap in a weighted/Huffman scheme only if real hint usage proves it matters.
fn easiest_labels(n: usize, alphabet: &[char]) -> Vec<String> {
    let mut labels: Vec<String> = alphabet.iter().map(|c| c.to_string()).collect();
    while labels.len() < n {
        let parent = labels.pop().expect("alphabet non-empty");
        for c in alphabet {
            labels.push(format!("{parent}{c}"));
        }
    }
    labels.truncate(n);
    labels
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_prefix_free(labels: &[String]) -> bool {
        for (i, a) in labels.iter().enumerate() {
            for (j, b) in labels.iter().enumerate() {
                if i != j && b.starts_with(a.as_str()) {
                    return false;
                }
            }
        }
        true
    }

    #[test]
    fn small_set_uses_easiest_single_keys() {
        let labels = assign(&[0, 0, 0]);
        assert_eq!(labels, vec!["f", "j", "d"]);
    }

    #[test]
    fn highest_priority_gets_easiest_key() {
        // element 2 is the most clickable; it must get 'f', the easiest key.
        let labels = assign(&[1, 5, 9, 3]);
        assert_eq!(labels[2], "f");
        assert_eq!(labels[1], "j");
    }

    #[test]
    fn ties_keep_reading_order() {
        let labels = assign(&[5, 5, 5]);
        assert_eq!(labels, vec!["f", "j", "d"]);
    }

    #[test]
    fn overflow_stays_prefix_free_and_complete() {
        let n = 80; // > 26, forces multi-char hints
        let labels = assign(&vec![0; n]);
        assert_eq!(labels.len(), n);
        assert!(is_prefix_free(&labels), "labels must be prefix-free");
        let uniq: std::collections::HashSet<_> = labels.iter().collect();
        assert_eq!(uniq.len(), n, "labels must be unique");
        // Top-priority element still gets a one-key hint.
        assert_eq!(labels[0].len(), 1);
    }

    #[test]
    fn empty_is_empty() {
        assert!(assign(&[]).is_empty());
    }
}
