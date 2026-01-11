/// Tests for single list operations
use list_utils::operations::single_list::{
    count_items, process_single_list, remove_duplicates, sort_ascending, sort_descending,
    trim_spaces,
};

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
fn test_sort_ascending() {
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
fn test_sort_descending() {
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
fn test_count_items() {
    let items = vec!["a".to_string(), "b".to_string(), "a".to_string()];
    let (total, unique) = count_items(&items);
    assert_eq!(total, 3);
    assert_eq!(unique, 2);
}

#[test]
fn test_process_single_list() {
    let items = vec![
        "  c  ".to_string(),
        "a".to_string(),
        "  c  ".to_string(),
        "b".to_string(),
    ];
    let result = process_single_list(&items, true, true, true, false);
    assert_eq!(result.items, vec!["a", "b", "c"]);
    assert_eq!(result.total_count, 3);
    assert_eq!(result.unique_count, 3);
}

#[test]
fn test_count_with_duplicates_at_end() {
    // Lista: 1, 2, 3, 4, 5, 6, 6, 6, 6
    // Total: 9 elementos
    // Únicos: 6 elementos (1, 2, 3, 4, 5, 6)
    let items = vec![
        "1".to_string(),
        "2".to_string(),
        "3".to_string(),
        "4".to_string(),
        "5".to_string(),
        "6".to_string(),
        "6".to_string(),
        "6".to_string(),
        "6".to_string(),
    ];

    let (total, unique) = count_items(&items);
    assert_eq!(total, 9);
    assert_eq!(unique, 6);
}

#[test]
fn test_process_list_with_duplicates_at_end() {
    // Lista: 1, 2, 3, 4, 5, 6, 6, 6, 6
    // Después de dedup: 1, 2, 3, 4, 5, 6 (6 elementos)
    let items = vec![
        "1".to_string(),
        "2".to_string(),
        "3".to_string(),
        "4".to_string(),
        "5".to_string(),
        "6".to_string(),
        "6".to_string(),
        "6".to_string(),
        "6".to_string(),
    ];

    // Sin trim, con dedup, sin ordenar
    let result = process_single_list(&items, false, true, false, false);

    assert_eq!(result.items, vec!["1", "2", "3", "4", "5", "6"]);
    assert_eq!(result.total_count, 6); // Después de dedup
    assert_eq!(result.unique_count, 6); // Todos únicos después de dedup
}
