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
/// A *balanced* code: pick the shortest length that fits `n`, then keep as many
/// labels as possible one char shorter — so the easiest keys are also the
/// shortest, and no label runs away long. With 26 keys every screen up to 676
/// elements gets labels of at most two chars, and lengths differ by at most one.
///
/// (The earlier version expanded the *hardest* label repeatedly, which piled all
/// the length onto the tail: past ~76 elements the worst hints ballooned to
/// `zzza`-style 4-char monsters that overlapped and were miserable to type.)
fn easiest_labels(n: usize, alphabet: &[char]) -> Vec<String> {
    if n == 0 {
        return Vec::new();
    }
    let k = alphabet.len();

    // No more targets than keys: one char each, easiest first.
    if n <= k {
        return (0..n).map(|i| alphabet[i].to_string()).collect();
    }

    // `len` = shortest label length that can hold n labels (k^len >= n).
    let mut len = 1usize;
    let mut cap = k;
    while cap < n {
        cap = cap.saturating_mul(k);
        len += 1;
    }

    // There are `parents` = k^(len-1) prefixes of length len-1. Keeping one as a
    // leaf costs one label; expanding it into its k children yields k. Expand
    // only the hardest few needed to reach n, so the easy prefixes stay short.
    let parents = k.pow((len - 1) as u32);
    let expand = (n - parents).div_ceil(k - 1); // prefixes to fan out
    let short = parents - expand; // prefixes kept as length-(len-1) leaves

    let mut labels = Vec::with_capacity(n);
    for i in 0..short {
        labels.push(seq(i, len - 1, alphabet));
    }
    for i in short..parents {
        let prefix = seq(i, len - 1, alphabet);
        for &c in alphabet {
            labels.push(format!("{prefix}{c}"));
        }
    }
    labels.truncate(n);
    labels
}

/// The `i`-th length-`m` sequence over `alphabet`, in odometer order (base-`k`
/// counting, most-significant char first). `seq(0, 2)` is "ff", `seq(1, 2)` is
/// "fj", etc. — so lower indices are the easier-to-type combinations.
fn seq(i: usize, m: usize, alphabet: &[char]) -> String {
    let k = alphabet.len();
    let mut idx = i;
    let mut chars = vec![alphabet[0]; m];
    for pos in (0..m).rev() {
        chars[pos] = alphabet[idx % k];
        idx /= k;
    }
    chars.into_iter().collect()
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
    fn large_set_stays_short_and_balanced() {
        // A busy window can have hundreds of elements; hints must not balloon.
        let n = 300;
        let labels = assign(&vec![0; n]);
        assert_eq!(labels.len(), n);
        assert!(is_prefix_free(&labels), "labels must be prefix-free");
        let uniq: std::collections::HashSet<_> = labels.iter().collect();
        assert_eq!(uniq.len(), n, "labels must be unique");

        let max = labels.iter().map(|l| l.len()).max().unwrap();
        let min = labels.iter().map(|l| l.len()).min().unwrap();
        assert_eq!(max, 2, "300 elements must fit in two keys, not more");
        assert!(max - min <= 1, "hint lengths must differ by at most one");
        // The highest-priority element still gets the single easiest key.
        assert_eq!(labels[0], "f");
    }

    #[test]
    fn full_two_char_space_is_uniform() {
        // Exactly k^2 elements: every label is a distinct two-key combo.
        let labels = assign(&vec![0; 26 * 26]);
        assert!(labels.iter().all(|l| l.len() == 2));
        assert!(is_prefix_free(&labels));
    }

    #[test]
    fn empty_is_empty() {
        assert!(assign(&[]).is_empty());
    }
}
