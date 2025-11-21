#[derive(Debug, Clone, PartialEq)]
pub enum ToonValue {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Array(Vec<ToonValue>),
    Map(Vec<(String, ToonValue)>), // Ordered map to match PHP array behavior
}

// --- Parser ---

pub fn parse(input: &str) -> anyhow::Result<ToonValue> {
    let lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() {
        return Ok(ToonValue::Null);
    }

    // Check if it's a single line value and not a key-value pair
    if lines.len() == 1 {
        let line = lines[0].trim();
        if !line.contains(':') && !line.is_empty() {
            return Ok(parse_value(line));
        }
    }

    parse_lines(&lines, 0, 0).map(|(val, _)| val)
}

fn parse_lines(
    lines: &[&str],
    start_idx: usize,
    base_indent: usize,
) -> anyhow::Result<(ToonValue, usize)> {
    parse_lines_impl(lines, start_idx, base_indent, 0)
}

fn parse_lines_impl(
    lines: &[&str],
    start_idx: usize,
    base_indent: usize,
    depth: usize,
) -> anyhow::Result<(ToonValue, usize)> {
    const MAX_PARSE_DEPTH: usize = 100;
    if depth > MAX_PARSE_DEPTH {
        return Err(anyhow::anyhow!("Parse depth limit exceeded"));
    }

    let mut map = Vec::new();
    let mut i = start_idx;

    while i < lines.len() {
        let line = lines[i];

        // Skip empty lines
        if line.trim().is_empty() {
            i += 1;
            continue;
        }

        let indent = line.len() - line.trim_start().len();
        if indent < base_indent {
            // End of current block
            break;
        }

        let trimmed = line.trim();
        if let Some((key_part, val_part)) = trimmed.split_once(':') {
            let key = key_part.trim().to_string();
            let val_str = val_part.trim();

            if val_str.is_empty() {
                // Nested object or empty
                // Check next line to see if it's a child
                if i + 1 < lines.len() {
                    let next_line = lines[i + 1];
                    if !next_line.trim().is_empty() {
                        let next_indent = next_line.len() - next_line.trim_start().len();

                        if next_indent > indent {
                            let (nested_val, consumed) =
                                parse_lines_impl(lines, i + 1, next_indent, depth + 1)?;
                            map.push((key, nested_val));
                            i = consumed;
                            continue;
                        }
                    }
                }
                // No children, treat as empty map (or null? spec is vague, assuming empty map for container)
                map.push((key, ToonValue::Map(Vec::new())));
            } else {
                // Inline value
                map.push((key, parse_value(val_str)));
            }
        } else {
            // Line without colon?
            // Could be a continuation or error. For now, ignore or treat as string key with null?
            // Spec says "Key-value pairs with colons".
        }
        i += 1;
    }

    Ok((ToonValue::Map(map), i))
}

fn parse_value(s: &str) -> ToonValue {
    let s = s.trim();
    if s == "true" {
        return ToonValue::Bool(true);
    }
    if s == "false" {
        return ToonValue::Bool(false);
    }
    if s == "null" {
        return ToonValue::Null;
    }
    if let Ok(i) = s.parse::<i64>() {
        return ToonValue::Int(i);
    }
    if let Ok(f) = s.parse::<f64>() {
        return ToonValue::Float(f);
    }

    // Handle quoted strings
    if s.starts_with('"') && s.ends_with('"') && s.len() >= 2 {
        // Simple unescape: replace \" with " and \\ with \
        // For a full implementation, use a proper unescape function
        let inner = &s[1..s.len() - 1];
        let unescaped = inner
            .replace("\\\"", "\"")
            .replace("\\n", "\n")
            .replace("\\\\", "\\");
        return ToonValue::String(unescaped);
    }

    // Handle lists: comma separated values
    // Only if it doesn't look like a string with commas (heuristics are hard)
    // For now, if it contains ',' and is not quoted, split it.
    if s.contains(',') {
        let parts: Vec<&str> = s.split(',').map(|p| p.trim()).collect();
        // Recursively parse items?
        // If we have "1, 2, 3", we want [Int(1), Int(2), Int(3)]
        // If we have "a, b, c", we want [String("a"), String("b"), String("c")]
        let mut items: Vec<ToonValue> = Vec::with_capacity(parts.len());
        for &p in parts.iter() {
            items.push(parse_value(p));
        }
        return ToonValue::Array(items);
    }

    ToonValue::String(s.to_string())
}

// --- Encoder ---

pub fn encode(val: &ToonValue) -> String {
    let mut out = String::new();
    encode_recursive_impl(val, 0, &mut out, 0);
    out.trim_end().to_string()
}

fn encode_recursive(val: &ToonValue, indent: usize, out: &mut String) {
    encode_recursive_impl(val, indent, out, 0);
}

fn encode_recursive_impl(val: &ToonValue, indent: usize, out: &mut String, depth: usize) {
    const MAX_ENCODE_DEPTH: usize = 100;
    if depth > MAX_ENCODE_DEPTH {
        out.push_str("[MaxDepthExceeded]");
        return;
    }

    let prefix = " ".repeat(indent);

    match val {
        ToonValue::Map(entries) => {
            for (key, value) in entries {
                out.push_str(&prefix);
                out.push_str(key);

                match value {
                    ToonValue::Map(_) => {
                        out.push_str(":\n");
                        encode_recursive_impl(value, indent + 2, out, depth + 1);
                    }
                    _ => {
                        out.push_str(": ");
                        encode_recursive_impl(value, 0, out, depth + 1); // 0 indent because it's inline
                    }
                }
            }
        }
        ToonValue::Array(items) => {
            let s = items
                .iter()
                .map(|v| value_to_string(v))
                .collect::<Vec<_>>()
                .join(", ");
            out.push_str(&s);
            out.push('\n');
        }
        _ => {
            out.push_str(&value_to_string(val));
            out.push('\n');
        }
    }
}

fn value_to_string(val: &ToonValue) -> String {
    match val {
        ToonValue::Null => "null".to_string(),
        ToonValue::Bool(b) => b.to_string(),
        ToonValue::Int(i) => i.to_string(),
        ToonValue::Float(f) => f.to_string(),
        ToonValue::String(s) => {
            // Always quote empty strings
            if s.is_empty() {
                return "\"\"".to_string();
            }
            // Quote if contains special chars
            if s.contains('\n')
                || s.contains(':')
                || s.contains(',')
                || s.contains('"')
                || s.trim() != s
            {
                // Simple escape
                let escaped = s
                    .replace('\\', "\\\\")
                    .replace('"', "\\\"")
                    .replace('\n', "\\n");
                format!("\"{}\"", escaped)
            } else {
                s.as_str().to_string()
            }
        }
        ToonValue::Array(items) => {
            let mut result = String::with_capacity(items.len() * 10);
            result.push('[');
            for (i, v) in items.iter().enumerate() {
                if i > 0 {
                    result.push_str(", ");
                }
                result.push_str(&value_to_string(v));
            }
            result.push(']');
            result
        }
        ToonValue::Map(_) => "[Object]".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ============================================================================
    // SECTION 1: BASIC PARSING TESTS
    // ============================================================================

    #[test]
    fn test_decode_simple() {
        let input = "user:\n  id: 123\n  email: ada@example.com";
        let val = parse(input).unwrap();

        if let ToonValue::Map(m) = val {
            assert_eq!(m.len(), 1);
            assert_eq!(m[0].0, "user");
            if let ToonValue::Map(user) = &m[0].1 {
                assert_eq!(user[0].0, "id");
                assert_eq!(user[0].1, ToonValue::Int(123));
                assert_eq!(user[1].0, "email");
                assert_eq!(user[1].1, ToonValue::String("ada@example.com".to_string()));
            } else {
                panic!("Expected user map");
            }
        } else {
            panic!("Expected map");
        }
    }

    #[test]
    fn test_parse_primitives() {
        assert_eq!(parse_value("null"), ToonValue::Null);
        assert_eq!(parse_value("true"), ToonValue::Bool(true));
        assert_eq!(parse_value("false"), ToonValue::Bool(false));
        assert_eq!(parse_value("42"), ToonValue::Int(42));
        assert_eq!(parse_value("-99"), ToonValue::Int(-99));
        assert_eq!(parse_value("3.14"), ToonValue::Float(3.14));
        assert_eq!(parse_value("hello"), ToonValue::String("hello".to_string()));
    }

    #[test]
    fn test_parse_quoted_strings() {
        assert_eq!(
            parse_value("\"hello world\""),
            ToonValue::String("hello world".to_string())
        );
        assert_eq!(
            parse_value("\"string with \\\"quotes\\\"\""),
            ToonValue::String("string with \"quotes\"".to_string())
        );
        assert_eq!(
            parse_value("\"path\\\\to\\\\file\""),
            ToonValue::String("path\\to\\file".to_string())
        );
        assert_eq!(
            parse_value("\"Line 1\\nLine 2\""),
            ToonValue::String("Line 1\nLine 2".to_string())
        );
    }

    #[test]
    fn test_parse_inline_arrays() {
        // Simple numeric array
        let result = parse_value("1, 2, 3, 4, 5");
        if let ToonValue::Array(items) = result {
            assert_eq!(items.len(), 5);
            assert_eq!(items[0], ToonValue::Int(1));
            assert_eq!(items[4], ToonValue::Int(5));
        } else {
            panic!("Expected array");
        }

        // Mixed types
        let result = parse_value("42, hello, true");
        if let ToonValue::Array(items) = result {
            assert_eq!(items.len(), 3);
            assert_eq!(items[0], ToonValue::Int(42));
            assert_eq!(items[1], ToonValue::String("hello".to_string()));
            assert_eq!(items[2], ToonValue::Bool(true));
        } else {
            panic!("Expected array");
        }
    }

    #[test]
    fn test_parse_deep_nesting() {
        let input = "l1:\n  l2:\n    l3:\n      l4:\n        l5: value";
        let val = parse(input).unwrap();

        // Navigate through nested structure
        if let ToonValue::Map(m) = val {
            assert_eq!(m.len(), 1);
            if let ToonValue::Map(l2) = &m[0].1 {
                if let ToonValue::Map(l3) = &l2[0].1 {
                    if let ToonValue::Map(l4) = &l3[0].1 {
                        if let ToonValue::Map(l5) = &l4[0].1 {
                            assert_eq!(l5[0].0, "l5");
                            assert_eq!(l5[0].1, ToonValue::String("value".to_string()));
                        } else {
                            panic!("Expected l4 to have map");
                        }
                    } else {
                        panic!("Expected l3 to have map");
                    }
                } else {
                    panic!("Expected l2 to have map");
                }
            } else {
                panic!("Expected l1 to have map");
            }
        } else {
            panic!("Expected root map");
        }
    }

    #[test]
    fn test_parse_multiple_keys() {
        let input = "name: Alice\nage: 30\nemail: alice@example.com";
        let val = parse(input).unwrap();

        if let ToonValue::Map(m) = val {
            assert_eq!(m.len(), 3);
            assert_eq!(m[0].0, "name");
            assert_eq!(m[0].1, ToonValue::String("Alice".to_string()));
            assert_eq!(m[1].0, "age");
            assert_eq!(m[1].1, ToonValue::Int(30));
            assert_eq!(m[2].0, "email");
            assert_eq!(m[2].1, ToonValue::String("alice@example.com".to_string()));
        } else {
            panic!("Expected map");
        }
    }

    // ============================================================================
    // SECTION 2: ENCODING TESTS
    // ============================================================================

    #[test]
    fn test_encode_simple() {
        let mut user_map = Vec::new();
        user_map.push(("id".to_string(), ToonValue::Int(123)));
        user_map.push((
            "email".to_string(),
            ToonValue::String("ada@example.com".to_string()),
        ));

        let mut root_map = Vec::new();
        root_map.push(("user".to_string(), ToonValue::Map(user_map)));

        let val = ToonValue::Map(root_map);
        let encoded = encode(&val);

        let expected = "user:\n  id: 123\n  email: ada@example.com";
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_encode_primitives() {
        assert_eq!(encode(&ToonValue::Null), "null");
        assert_eq!(encode(&ToonValue::Bool(true)), "true");
        assert_eq!(encode(&ToonValue::Bool(false)), "false");
        assert_eq!(encode(&ToonValue::Int(42)), "42");
        assert_eq!(encode(&ToonValue::Int(-99)), "-99");
        assert_eq!(encode(&ToonValue::Float(3.14)), "3.14");
        assert_eq!(encode(&ToonValue::String("hello".to_string())), "hello");
    }

    #[test]
    fn test_encode_quoted_strings() {
        // Strings with newlines should be quoted
        let val = ToonValue::String("Line 1\nLine 2".to_string());
        let encoded = encode(&val);
        assert!(encoded.contains('"') && encoded.contains("\\n"));

        // Strings with colons should be quoted
        let val = ToonValue::String("key: value".to_string());
        let encoded = encode(&val);
        assert!(encoded.contains('"'));

        // Strings with quotes should be escaped
        let val = ToonValue::String("He said \"hello\"".to_string());
        let encoded = encode(&val);
        assert!(encoded.contains("\\\""));
    }

    #[test]
    fn test_encode_arrays() {
        let items = vec![ToonValue::Int(1), ToonValue::Int(2), ToonValue::Int(3)];
        let val = ToonValue::Array(items);
        let encoded = encode(&val);
        assert!(encoded.contains("1, 2, 3"));
    }

    #[test]
    fn test_encode_mixed_array_types() {
        let items = vec![
            ToonValue::Int(42),
            ToonValue::String("hello".to_string()),
            ToonValue::Bool(true),
            ToonValue::Null,
        ];
        let val = ToonValue::Array(items);
        let encoded = encode(&val);
        assert!(encoded.contains("42"));
        assert!(encoded.contains("hello"));
        assert!(encoded.contains("true"));
        assert!(encoded.contains("null"));
    }

    // ============================================================================
    // SECTION 3: ROUNDTRIP TESTS (ENCODE -> DECODE CONSISTENCY)
    // ============================================================================

    #[test]
    fn test_roundtrip_simple_map() {
        let input = "user:\n  id: 123\n  email: ada@example.com";
        let parsed = parse(input).unwrap();
        let encoded = encode(&parsed);
        let reparsed = parse(&encoded).unwrap();
        assert_eq!(parsed, reparsed);
    }

    #[test]
    fn test_roundtrip_nested_structure() {
        let input = "company:\n  name: TechCorp\n  departments:\n    engineering:\n      employees: 10\n    sales:\n      employees: 5";
        let parsed = parse(input).unwrap();
        let encoded = encode(&parsed);
        let reparsed = parse(&encoded).unwrap();
        assert_eq!(parsed, reparsed);
    }

    #[test]
    fn test_roundtrip_arrays() {
        let items = vec![ToonValue::Int(1), ToonValue::Int(2), ToonValue::Int(3)];
        let val = ToonValue::Array(items.clone());
        let encoded = encode(&val);
        let decoded = parse(&encoded).unwrap();
        assert_eq!(ToonValue::Array(items), decoded);
    }

    #[test]
    fn test_full_example() {
        let input = "user:\n  id: 123\n  email: ada@example.com\n  metadata:\n    active: true\n    score: 9.5";
        let val = parse(input).unwrap();
        let encoded = encode(&val);
        assert_eq!(encoded, input);
    }

    // ============================================================================
    // SECTION 4: EDGE CASES AND SPECIAL CHARACTERS
    // ============================================================================

    #[test]
    fn test_empty_string_key() {
        let mut map = Vec::new();
        map.push(("empty_value".to_string(), ToonValue::String("".to_string())));
        let val = ToonValue::Map(map);
        let encoded = encode(&val);
        assert!(encoded.contains("empty_value:") && encoded.contains("\"\""));
    }

    #[test]
    fn test_special_characters_in_string() {
        let test_strings = vec!["!@#$%^&*()", "path/to/file"];

        for test_str in test_strings {
            let val = ToonValue::String(test_str.to_string());
            let encoded = encode(&val);
            let decoded = parse(&encoded).unwrap();

            if let ToonValue::String(decoded_str) = decoded {
                assert_eq!(test_str, decoded_str, "Failed roundtrip for: {}", test_str);
            } else {
                panic!("Expected string, got {:?}", decoded);
            }
        }
    }

    #[test]
    fn test_unicode_characters() {
        let unicode_strings = vec!["Hello 🌍", "你好世界", "مرحبا بالعالم", "Привет мир"];

        for test_str in unicode_strings {
            let val = ToonValue::String(test_str.to_string());
            let encoded = encode(&val);
            let decoded = parse(&encoded).unwrap();

            if let ToonValue::String(decoded_str) = decoded {
                assert_eq!(test_str, decoded_str, "Failed roundtrip for: {}", test_str);
            } else {
                panic!("Expected string, got {:?}", decoded);
            }
        }
    }

    #[test]
    fn test_large_integers() {
        let large_int = i64::MAX;
        let val = ToonValue::Int(large_int);
        let encoded = encode(&val);
        let decoded = parse(&encoded).unwrap();
        assert_eq!(val, decoded);

        let neg_large_int = i64::MIN;
        let val = ToonValue::Int(neg_large_int);
        let encoded = encode(&val);
        let decoded = parse(&encoded).unwrap();
        assert_eq!(val, decoded);
    }

    #[test]
    fn test_float_precision() {
        let float_val = 3.141592653589793;
        let val = ToonValue::Float(float_val);
        let encoded = encode(&val);
        let decoded = parse(&encoded).unwrap();
        if let ToonValue::Float(f) = decoded {
            assert!((float_val - f).abs() < 0.0001);
        } else {
            panic!("Expected float");
        }
    }

    #[test]
    fn test_parse_empty_input() {
        let result = parse("");
        assert_eq!(result.unwrap(), ToonValue::Null);
    }

    #[test]
    fn test_map_with_many_entries() {
        let mut map = Vec::new();
        for i in 1..=50 {
            map.push((format!("key{}", i), ToonValue::Int(i as i64)));
        }
        let val = ToonValue::Map(map);
        let encoded = encode(&val);
        let decoded = parse(&encoded).unwrap();
        assert_eq!(val, decoded);
    }

    #[test]
    fn test_deeply_nested_arrays() {
        let inner = vec![ToonValue::Int(1), ToonValue::Int(2)];
        let middle = vec![ToonValue::Array(inner)];
        let outer = vec![ToonValue::Array(middle)];
        let val = ToonValue::Array(outer);

        let encoded = encode(&val);
        // Verify nested arrays are encoded correctly
        assert!(encoded.contains("["));
    }
}
