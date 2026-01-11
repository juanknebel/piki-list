/// Operations for comparing two lists

/// Options for list comparison
#[derive(Debug, Clone, Copy)]
pub struct CompareOptions {
    /// Whether comparison should be case-sensitive
    pub case_sensitive: bool,
    /// Whether to trim spaces before comparison
    pub trim_spaces: bool,
}

impl Default for CompareOptions {
    fn default() -> Self {
        Self {
            case_sensitive: false,
            trim_spaces: true,
        }
    }
}

/// Result of comparing two lists
#[derive(Debug, Clone)]
pub struct CompareResult {
    /// Items only in the first list
    pub only_in_first: Vec<String>,
    /// Items only in the second list
    pub only_in_second: Vec<String>,
    /// Items in both lists (intersection)
    pub intersection: Vec<String>,
    /// All unique items from both lists (union)
    pub union: Vec<String>,
}

/// Check if all items can be parsed as numbers (integers or floats)
fn all_numeric(items: &[String]) -> bool {
    !items.is_empty() && items.iter().all(|s| s.trim().parse::<f64>().is_ok())
}

/// Sort items intelligently (numeric if all numbers, otherwise alphabetic)
fn sort_items_smart(items: &mut [String]) {
    if all_numeric(items) {
        // Numeric sort
        items.sort_by(|a, b| {
            let a_num: f64 = a.trim().parse().unwrap_or(0.0);
            let b_num: f64 = b.trim().parse().unwrap_or(0.0);
            a_num
                .partial_cmp(&b_num)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
    } else {
        // Alphabetic sort
        items.sort();
    }
}

/// Normalize an item according to comparison options
fn normalize_item(item: &str, options: CompareOptions) -> String {
    let mut normalized = item.to_string();
    if options.trim_spaces {
        normalized = normalized.trim().to_string();
    }
    if !options.case_sensitive {
        normalized = normalized.to_lowercase();
    }
    normalized
}

/// Compare two lists and return the differences and common elements
///
/// # Arguments
/// * `list1` - First list of items
/// * `list2` - Second list of items
/// * `options` - Comparison options
///
/// # Returns
/// CompareResult with all comparison results
pub fn compare_lists(list1: &[String], list2: &[String], options: CompareOptions) -> CompareResult {
    // Normalize items according to options
    let normalized1: Vec<(String, String)> = list1
        .iter()
        .map(|item| (normalize_item(item, options), item.clone()))
        .collect();
    let normalized2: Vec<(String, String)> = list2
        .iter()
        .map(|item| (normalize_item(item, options), item.clone()))
        .collect();

    // Create sets for efficient lookup
    let set1: std::collections::HashSet<String> =
        normalized1.iter().map(|(n, _)| n.clone()).collect();
    let set2: std::collections::HashSet<String> =
        normalized2.iter().map(|(n, _)| n.clone()).collect();

    // Find items only in first list
    let mut only_in_first: Vec<String> = normalized1
        .iter()
        .filter(|(normalized, _)| !set2.contains(normalized))
        .map(|(_, original)| original.clone())
        .collect();

    // Find items only in second list
    let mut only_in_second: Vec<String> = normalized2
        .iter()
        .filter(|(normalized, _)| !set1.contains(normalized))
        .map(|(_, original)| original.clone())
        .collect();

    // Find intersection
    let mut intersection: Vec<String> = normalized1
        .iter()
        .filter(|(normalized, _)| set2.contains(normalized))
        .map(|(_, original)| original.clone())
        .collect();

    // Find union (all unique items)
    let mut union_set = std::collections::HashSet::new();
    for (_, original) in &normalized1 {
        union_set.insert(original.clone());
    }
    for (_, original) in &normalized2 {
        union_set.insert(original.clone());
    }
    let mut union: Vec<String> = union_set.into_iter().collect();

    // Sort all result vectors intelligently (numeric if all numbers, otherwise alphabetic)
    sort_items_smart(&mut only_in_first);
    sort_items_smart(&mut only_in_second);
    sort_items_smart(&mut intersection);
    sort_items_smart(&mut union);

    CompareResult {
        only_in_first,
        only_in_second,
        intersection,
        union,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare_basic() {
        let list1 = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let list2 = vec!["b".to_string(), "c".to_string(), "d".to_string()];
        let options = CompareOptions::default();
        let result = compare_lists(&list1, &list2, options);

        assert_eq!(result.only_in_first, vec!["a"]);
        assert_eq!(result.only_in_second, vec!["d"]);
        assert_eq!(result.intersection.len(), 2);
        assert!(result.intersection.contains(&"b".to_string()));
        assert!(result.intersection.contains(&"c".to_string()));
    }

    #[test]
    fn test_compare_case_insensitive() {
        let list1 = vec!["A".to_string(), "b".to_string()];
        let list2 = vec!["a".to_string(), "B".to_string()];
        let options = CompareOptions {
            case_sensitive: false,
            trim_spaces: false,
        };
        let result = compare_lists(&list1, &list2, options);

        assert_eq!(result.only_in_first.len(), 0);
        assert_eq!(result.only_in_second.len(), 0);
        assert_eq!(result.intersection.len(), 2);
    }

    #[test]
    fn test_compare_case_sensitive() {
        let list1 = vec!["A".to_string(), "b".to_string()];
        let list2 = vec!["a".to_string(), "B".to_string()];
        let options = CompareOptions {
            case_sensitive: true,
            trim_spaces: false,
        };
        let result = compare_lists(&list1, &list2, options);

        assert_eq!(result.only_in_first.len(), 2);
        assert_eq!(result.only_in_second.len(), 2);
        assert_eq!(result.intersection.len(), 0);
    }

    #[test]
    fn test_compare_trim_spaces() {
        let list1 = vec!["  a  ".to_string(), "b".to_string()];
        let list2 = vec!["a".to_string(), "  b  ".to_string()];
        let options = CompareOptions {
            case_sensitive: false,
            trim_spaces: true,
        };
        let result = compare_lists(&list1, &list2, options);

        assert_eq!(result.only_in_first.len(), 0);
        assert_eq!(result.only_in_second.len(), 0);
        assert_eq!(result.intersection.len(), 2);
    }

    #[test]
    fn test_compare_numeric_sorting() {
        // Test that numeric results are sorted numerically, not alphabetically
        let list1 = vec![
            "10".to_string(),
            "9".to_string(),
            "11".to_string(),
            "4".to_string(),
        ];
        let list2 = vec!["5".to_string(), "9".to_string(), "12".to_string()];
        let options = CompareOptions::default();
        let result = compare_lists(&list1, &list2, options);

        // Only in List 1 should be sorted numerically: 4, 10, 11 (not 10, 11, 4)
        assert_eq!(result.only_in_first, vec!["4", "10", "11"]);

        // Only in List 2 should be sorted numerically: 5, 12
        assert_eq!(result.only_in_second, vec!["5", "12"]);

        // Intersection should be sorted numerically: 9
        assert_eq!(result.intersection, vec!["9"]);

        // Union should be sorted numerically: 4, 5, 9, 10, 11, 12
        assert_eq!(result.union, vec!["4", "5", "9", "10", "11", "12"]);
    }
}
