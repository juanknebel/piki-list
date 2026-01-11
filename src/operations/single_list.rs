/// Operations for single list manipulation

/// Result of single list operations
#[derive(Debug, Clone)]
pub struct SingleListResult {
    /// The processed items
    pub items: Vec<String>,
    /// Total count of items
    #[allow(dead_code)]
    pub total_count: usize,
    /// Count of unique items
    #[allow(dead_code)]
    pub unique_count: usize,
}

/// Trim whitespace from all items in a list
///
/// # Arguments
/// * `items` - Vector of items to trim
///
/// # Returns
/// New vector with trimmed items
pub fn trim_spaces(items: &[String]) -> Vec<String> {
    items.iter().map(|s| s.trim().to_string()).collect()
}

/// Remove duplicate items from a list, preserving order
///
/// # Arguments
/// * `items` - Vector of items to deduplicate
///
/// # Returns
/// New vector without duplicates
pub fn remove_duplicates(items: &[String]) -> Vec<String> {
    let mut seen = std::collections::HashSet::new();
    items
        .iter()
        .filter(|item| seen.insert((*item).clone()))
        .cloned()
        .collect()
}

/// Check if all items can be parsed as numbers (integers or floats)
fn all_numeric(items: &[String]) -> bool {
    !items.is_empty() && items.iter().all(|s| s.trim().parse::<f64>().is_ok())
}

/// Sort items in ascending order
/// If all items are numeric, sorts numerically; otherwise sorts alphabetically
///
/// # Arguments
/// * `items` - Vector of items to sort
///
/// # Returns
/// New sorted vector
pub fn sort_ascending(items: &[String]) -> Vec<String> {
    let mut sorted = items.to_vec();

    if all_numeric(&sorted) {
        // Numeric sort
        sorted.sort_by(|a, b| {
            let a_num: f64 = a.trim().parse().unwrap_or(0.0);
            let b_num: f64 = b.trim().parse().unwrap_or(0.0);
            a_num
                .partial_cmp(&b_num)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
    } else {
        // Alphabetic sort
        sorted.sort();
    }

    sorted
}

/// Sort items in descending order
/// If all items are numeric, sorts numerically; otherwise sorts alphabetically
///
/// # Arguments
/// * `items` - Vector of items to sort
///
/// # Returns
/// New sorted vector (descending)
pub fn sort_descending(items: &[String]) -> Vec<String> {
    let mut sorted = items.to_vec();

    if all_numeric(&sorted) {
        // Numeric sort descending
        sorted.sort_by(|a, b| {
            let a_num: f64 = a.trim().parse().unwrap_or(0.0);
            let b_num: f64 = b.trim().parse().unwrap_or(0.0);
            b_num
                .partial_cmp(&a_num)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
    } else {
        // Alphabetic sort descending
        sorted.sort_by(|a, b| b.cmp(a));
    }

    sorted
}

/// Count total and unique items
///
/// # Arguments
/// * `items` - Vector of items to count
///
/// # Returns
/// Tuple of (total_count, unique_count)
pub fn count_items(items: &[String]) -> (usize, usize) {
    let total = items.len();
    let unique = items.iter().collect::<std::collections::HashSet<_>>().len();
    (total, unique)
}

/// Apply all operations to a list and return results
///
/// # Arguments
/// * `items` - Vector of items to process
/// * `trim` - Whether to trim spaces
/// * `dedup` - Whether to remove duplicates
/// * `sort_asc` - Whether to sort ascending (takes precedence over sort_desc)
/// * `sort_desc` - Whether to sort descending
///
/// # Returns
/// SingleListResult with processed items and counts
pub fn process_single_list(
    items: &[String],
    trim: bool,
    dedup: bool,
    sort_asc: bool,
    sort_desc: bool,
) -> SingleListResult {
    let mut processed = items.to_vec();

    if trim {
        processed = trim_spaces(&processed);
    }

    if dedup {
        processed = remove_duplicates(&processed);
    }

    if sort_asc {
        processed = sort_ascending(&processed);
    } else if sort_desc {
        processed = sort_descending(&processed);
    }

    let (total_count, unique_count) = count_items(&processed);

    SingleListResult {
        items: processed,
        total_count,
        unique_count,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trim_spaces() {
        let items = vec![
            "  item1  ".to_string(),
            "item2".to_string(),
            "  item3  ".to_string(),
        ];
        let result = trim_spaces(&items);
        assert_eq!(result, vec!["item1", "item2", "item3"]);
    }

    #[test]
    fn test_remove_duplicates() {
        let items = vec![
            "a".to_string(),
            "b".to_string(),
            "a".to_string(),
            "c".to_string(),
        ];
        let result = remove_duplicates(&items);
        assert_eq!(result, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_sort_ascending_alphabetic() {
        let items = vec!["c".to_string(), "a".to_string(), "b".to_string()];
        let result = sort_ascending(&items);
        assert_eq!(result, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_sort_ascending_numeric() {
        // Should sort as numbers: 4, 9, 10, 11 (not alphabetically: 10, 11, 4, 9)
        let items = vec![
            "10".to_string(),
            "9".to_string(),
            "11".to_string(),
            "4".to_string(),
        ];
        let result = sort_ascending(&items);
        assert_eq!(result, vec!["4", "9", "10", "11"]);
    }

    #[test]
    fn test_sort_descending_alphabetic() {
        let items = vec!["a".to_string(), "c".to_string(), "b".to_string()];
        let result = sort_descending(&items);
        assert_eq!(result, vec!["c", "b", "a"]);
    }

    #[test]
    fn test_sort_descending_numeric() {
        let items = vec![
            "10".to_string(),
            "9".to_string(),
            "11".to_string(),
            "4".to_string(),
        ];
        let result = sort_descending(&items);
        assert_eq!(result, vec!["11", "10", "9", "4"]);
    }

    #[test]
    fn test_sort_mixed_falls_back_to_alphabetic() {
        // Mixed numbers and text should sort alphabetically
        let items = vec!["10".to_string(), "abc".to_string(), "2".to_string()];
        let result = sort_ascending(&items);
        assert_eq!(result, vec!["10", "2", "abc"]);
    }

    #[test]
    fn test_count_items() {
        let items = vec!["a".to_string(), "b".to_string(), "a".to_string()];
        let (total, unique) = count_items(&items);
        assert_eq!(total, 3);
        assert_eq!(unique, 2);
    }
}
