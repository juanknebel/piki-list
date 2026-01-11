/// Tests for list comparison operations
use list_utils::operations::compare::{compare_lists, CompareOptions};

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
fn test_compare_union() {
    let list1 = vec!["a".to_string(), "b".to_string()];
    let list2 = vec!["b".to_string(), "c".to_string()];
    let options = CompareOptions::default();
    let result = compare_lists(&list1, &list2, options);

    assert_eq!(result.union.len(), 3);
    assert!(result.union.contains(&"a".to_string()));
    assert!(result.union.contains(&"b".to_string()));
    assert!(result.union.contains(&"c".to_string()));
}
