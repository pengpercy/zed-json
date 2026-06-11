use serde_json::Value;

pub fn trim_json_text(text: &str) -> &str {
    text.trim().trim_matches(['\n', '\t', '\r'])
}

pub fn parse_valid_json(text: &str) -> Option<Value> {
    let value = serde_json::from_str::<Value>(trim_json_text(text)).ok()?;
    match value {
        Value::Object(_) | Value::Array(_) | Value::Null => Some(value),
        _ => None,
    }
}

pub fn is_valid(text: &str) -> bool {
    parse_valid_json(text).is_some()
}

pub fn escape(text: &str) -> String {
    let trimmed = trim_json_text(text);
    if !is_valid(trimmed) {
        return trimmed.to_string();
    }

    serde_json::to_string(trimmed)
        .unwrap_or_else(|_| trimmed.to_string())
        .trim_start_matches('"')
        .trim_end_matches('"')
        .to_string()
}

pub fn unescape(text: &str) -> String {
    let trimmed = trim_json_text(text);
    let mut quoted = trimmed.to_string();

    if !trimmed.starts_with('"') {
        quoted.insert(0, '"');
    }

    if !trimmed.ends_with('"') {
        quoted.push('"');
    }

    serde_json::from_str::<String>(&quoted).unwrap_or_else(|_| trimmed.to_string())
}

pub fn beautify(text: &str, indent_width: usize) -> String {
    let trimmed = trim_json_text(text);
    let Some(value) = parse_valid_json(trimmed) else {
        return trimmed.to_string();
    };

    let pretty = serde_json::to_string_pretty(&value).unwrap_or_else(|_| trimmed.to_string());
    if indent_width == 2 {
        return pretty;
    }

    let indent = " ".repeat(indent_width.max(1));
    pretty.replace("\n  ", &format!("\n{indent}"))
}

pub fn uglify(text: &str) -> String {
    let trimmed = trim_json_text(text);
    let Some(value) = parse_valid_json(trimmed) else {
        return trimmed.to_string();
    };

    serde_json::to_string(&value).unwrap_or_else(|_| trimmed.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_json_objects_arrays_and_null() {
        assert!(is_valid(r#"{"a":1}"#));
        assert!(is_valid(r#"[1,2]"#));
        assert!(is_valid("null"));
        assert!(!is_valid(r#""text""#));
        assert!(!is_valid("42"));
        assert!(!is_valid("{"));
    }

    #[test]
    fn transforms_json_text() {
        assert_eq!(beautify(r#"{"a":1}"#, 2), "{\n  \"a\": 1\n}");
        assert_eq!(uglify("{\n  \"a\": 1\n}"), r#"{"a":1}"#);
        assert_eq!(escape(r#"{"a":"b"}"#), r#"{\"a\":\"b\"}"#);
        assert_eq!(unescape(r#"{\"a\":\"b\"}"#), r#"{"a":"b"}"#);
    }

    #[test]
    fn leaves_invalid_text_unchanged_after_trimming() {
        assert_eq!(beautify("  nope  ", 2), "nope");
        assert_eq!(uglify("  nope  "), "nope");
        assert_eq!(escape("  nope  "), "nope");
        assert_eq!(unescape("  \\q  "), "\\q");
    }
}
