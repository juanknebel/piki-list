use regex::Regex;
use serde_json;
use std::collections::BTreeSet;
/// Supported delimiters for parsing lists
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Delimiter {
    /// Newline character (\n)
    Newline,
    /// Tab character (\t)
    Tab,
    /// Comma (,)
    Comma,
    /// Semicolon (;)
    Semicolon,
    /// JSON format (auto-detected list of objects)
    Json,
}

impl Delimiter {
    /// Get the character representation of the delimiter
    pub fn as_char(&self) -> char {
        match self {
            Delimiter::Newline => '\n',
            Delimiter::Tab => '\t',
            Delimiter::Comma => ',',
            Delimiter::Semicolon => ';',
            Delimiter::Json => '{', // Logic will handle this specially
        }
    }

    /// Get a display string for the delimiter
    pub fn display_name(&self) -> &'static str {
        match self {
            Delimiter::Newline => "\\n",
            Delimiter::Tab => "\\t",
            Delimiter::Comma => ",",
            Delimiter::Semicolon => ";",
            Delimiter::Json => "JSON",
        }
    }

    /// Cycle to the next delimiter
    pub fn next(&self) -> Self {
        match self {
            Delimiter::Newline => Delimiter::Tab,
            Delimiter::Tab => Delimiter::Comma,
            Delimiter::Comma => Delimiter::Semicolon,
            Delimiter::Semicolon => Delimiter::Json,
            Delimiter::Json => Delimiter::Newline,
        }
    }
}

/// Parse a string into a vector of items using the specified delimiter
///
/// # Arguments
/// * `input` - The input string to parse
/// * `delimiter` - The delimiter to use for splitting
///
/// # Returns
/// A vector of strings, each representing an item from the list.
/// Ignores trailing empty element if input ends with delimiter.
pub fn parse_list(input: &str, delimiter: Delimiter) -> Vec<String> {
    if input.is_empty() {
        return Vec::new();
    }

    // Normalize Windows line endings to avoid empty items when pasting CRLF text
    let normalized = normalize_line_endings(input);

    let mut items: Vec<String> = normalized
        .split(delimiter.as_char())
        .map(|s| s.to_string())
        .collect();

    // Remove last element if it's empty (input ended with delimiter)
    if let Some(last) = items.last() {
        if last.is_empty() {
            items.pop();
        }
    }

    items
}

/// Parse a string as JSON and convert to a list of items.
/// Returns (list_of_items, repaired_json_string)
pub fn parse_json_to_list(input: &str, target_sep: char) -> Result<(Vec<String>, String), String> {
    if input.trim().is_empty() {
        return Ok((Vec::new(), String::new()));
    }

    let repaired = repair_json(input);
    let v: serde_json::Value = serde_json::from_str(&repaired).map_err(|e| {
        if repaired != input {
            format!("JSON Error (after auto-repair): {}", e)
        } else {
            e.to_string()
        }
    })?;

    // Treat single object as a 1-element array
    let arr = if let Some(a) = v.as_array() {
        a.clone()
    } else if v.is_object() {
        vec![v]
    } else {
        return Err("JSON input must be an array or a single object".to_string());
    };

    if arr.is_empty() {
        return Ok((Vec::new(), repaired));
    }

    // Check if first element is an object
    if let Some(_) = arr[0].as_object() {
        // It's a list of objects -> convert to CSV lines
        let mut csv_lines = Vec::new();

        // Get all unique keys from all objects
        let mut keys = BTreeSet::new();
        for item in &arr {
            if let Some(obj) = item.as_object() {
                for key in obj.keys() {
                    keys.insert(key.clone());
                }
            }
        }
        let keys_vec: Vec<String> = keys.into_iter().collect();

        // Header row
        let sep_str = target_sep.to_string();
        csv_lines.push(keys_vec.join(&sep_str));

        // Data rows
        for item in &arr {
            if let Some(obj) = item.as_object() {
                let row: Vec<String> = keys_vec
                    .iter()
                    .map(|k| match obj.get(k) {
                        Some(val) => {
                            if val.is_string() {
                                val.as_str().unwrap().to_string()
                            } else {
                                val.to_string()
                            }
                        }
                        None => "".to_string(),
                    })
                    .collect();
                csv_lines.push(row.join(&sep_str));
            }
        }
        Ok((csv_lines, repaired))
    } else {
        // It's a list of primitives -> just convert each to string
        let items: Vec<String> = arr
            .iter()
            .map(|v| {
                if v.is_string() {
                    v.as_str().unwrap().to_string()
                } else {
                    v.to_string()
                }
            })
            .collect();
        Ok((items, repaired))
    }
}

/// Helper to wrap unquoted keys in double quotes to support 'Lax JSON'
fn repair_json(input: &str) -> String {
    // Regex that matches unquoted keys:
    // It looks for a word followed by a colon, preceded by {, [ or , (or start of string)
    // We escape [ as \[
    let re = Regex::new(r"([{\[,]\s*)([a-zA-Z_][a-zA-Z0-9_]*)\s*:").unwrap();
    let res = re.replace_all(input, "$1\"$2\":").to_string();

    // Also handle the very first key if it starts with the key directly
    let re_start = Regex::new(r"^(\s*)([a-zA-Z_][a-zA-Z0-9_]*)\s*:").unwrap();
    re_start.replace(&res, "$1\"$2\":").to_string()
}

/// Replace CRLF/CR with LF to keep parsing consistent across platforms
fn normalize_line_endings(input: &str) -> String {
    let without_crlf = input.replace("\r\n", "\n");
    if without_crlf.contains('\r') {
        without_crlf.replace('\r', "\n")
    } else {
        without_crlf
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_delimiter_cycle() {
        let d = Delimiter::Newline;
        assert_eq!(d.next(), Delimiter::Tab);
        assert_eq!(d.next().next(), Delimiter::Comma);
        assert_eq!(d.next().next().next(), Delimiter::Semicolon);
        assert_eq!(d.next().next().next().next(), Delimiter::Newline);
    }

    #[test]
    fn test_parse_trailing_delimiter() {
        // Input ending with newline should not create empty last element
        let input = "item1\nitem2\nitem3\n";
        let result = parse_list(input, Delimiter::Newline);
        assert_eq!(result, vec!["item1", "item2", "item3"]);
    }

    #[test]
    fn test_parse_trailing_comma() {
        let input = "a,b,c,";
        let result = parse_list(input, Delimiter::Comma);
        assert_eq!(result, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_parse_crlf_normalization() {
        let input = "item1\r\nitem2\r\nitem3\r\n";
        let result = parse_list(input, Delimiter::Newline);
        assert_eq!(result, vec!["item1", "item2", "item3"]);
    }

    #[test]
    fn test_json_to_csv() {
        let input = "[{\"a\":1,\"b\":2},{\"a\":3,\"b\":5}]";
        let (result, _) = parse_json_to_list(input, ',').unwrap();
        assert_eq!(result, vec!["a,b", "1,2", "3,5"]);
    }

    #[test]
    fn test_lax_json() {
        // Unquoted keys should now be auto-repaired and valid
        let input = "[{a:1,b:2}]";
        let (result, repaired) = parse_json_to_list(input, ',').unwrap();
        assert_eq!(result, vec!["a,b", "1,2"]);
        assert!(repaired.contains("\"a\""));
    }
}
