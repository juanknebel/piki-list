/// Tests for the parser module
use list_utils::parser::{parse_list, Delimiter};

#[test]
fn test_parse_newline() {
    let input = "item1\nitem2\nitem3";
    let result = parse_list(input, Delimiter::Newline);
    assert_eq!(result, vec!["item1", "item2", "item3"]);
}

#[test]
fn test_parse_comma() {
    let input = "item1,item2,item3";
    let result = parse_list(input, Delimiter::Comma);
    assert_eq!(result, vec!["item1", "item2", "item3"]);
}

#[test]
fn test_parse_tab() {
    let input = "item1\titem2\titem3";
    let result = parse_list(input, Delimiter::Tab);
    assert_eq!(result, vec!["item1", "item2", "item3"]);
}

#[test]
fn test_parse_semicolon() {
    let input = "item1;item2;item3";
    let result = parse_list(input, Delimiter::Semicolon);
    assert_eq!(result, vec!["item1", "item2", "item3"]);
}

#[test]
fn test_parse_empty() {
    let result = parse_list("", Delimiter::Newline);
    assert_eq!(result, Vec::<String>::new());
}

#[test]
fn test_parse_with_whitespace() {
    let input = "  item1  \n  item2  \n  item3  ";
    let result = parse_list(input, Delimiter::Newline);
    assert_eq!(result, vec!["  item1  ", "  item2  ", "  item3  "]);
}
