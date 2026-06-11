use serde_json::Value;

pub fn trim_json_text(text: &str) -> &str {
    text.trim().trim_matches(['\n', '\t', '\r'])
}

fn is_object_like(value: &Value) -> bool {
    match value {
        Value::Object(_) | Value::Array(_) | Value::Null => true,
        _ => false,
    }
}

pub fn parse_valid_json(text: &str) -> Option<Value> {
    let value = serde_json::from_str::<Value>(trim_json_text(text)).ok()?;
    is_object_like(&value).then_some(value)
}

pub fn validate(text: &str) -> Result<(), String> {
    let value =
        serde_json::from_str::<Value>(trim_json_text(text)).map_err(|error| error.to_string())?;
    if is_object_like(&value) {
        Ok(())
    } else {
        Err("expected a JSON object, array, or null".to_string())
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

fn reindent_pretty_json(pretty: &str, indent: &str) -> String {
    let mut output = String::with_capacity(pretty.len());
    let mut chars = pretty.chars().peekable();

    while let Some(ch) = chars.next() {
        output.push(ch);
        if ch != '\n' {
            continue;
        }

        let mut depth = 0;
        while chars.next_if_eq(&' ').is_some() {
            depth += 1;
        }

        for _ in 0..(depth / 2) {
            output.push_str(indent);
        }
    }

    output
}

pub fn beautify_with_indent(text: &str, indent: &str) -> String {
    let trimmed = trim_json_text(text);
    let Some(value) = parse_valid_json(trimmed) else {
        return trimmed.to_string();
    };

    let pretty = serde_json::to_string_pretty(&value).unwrap_or_else(|_| trimmed.to_string());
    if indent == "  " {
        return pretty;
    }

    reindent_pretty_json(&pretty, indent)
}

pub fn beautify(text: &str, indent_width: usize) -> String {
    let indent = " ".repeat(indent_width.max(1));
    beautify_with_indent(text, &indent)
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
        assert_eq!(
            beautify(r#"{"a":{"b":1}}"#, 4),
            "{\n    \"a\": {\n        \"b\": 1\n    }\n}"
        );
        assert_eq!(
            beautify_with_indent(r#"{"a":{"b":1}}"#, "\t"),
            "{\n\t\"a\": {\n\t\t\"b\": 1\n\t}\n}"
        );
        assert_eq!(uglify("{\n  \"a\": 1\n}"), r#"{"a":1}"#);
        assert_eq!(escape(r#"{"a":"b"}"#), r#"{\"a\":\"b\"}"#);
        assert_eq!(unescape(r#"{\"a\":\"b\"}"#), r#"{"a":"b"}"#);
    }

    #[test]
    fn preserves_arbitrary_precision_numbers() {
        let json = r#"{"id":9007199254740993123456789}"#;
        assert_eq!(uglify(json), json);
        assert_eq!(
            beautify(json, 2),
            "{\n  \"id\": 9007199254740993123456789\n}"
        );
    }

    #[test]
    fn leaves_invalid_text_unchanged_after_trimming() {
        assert_eq!(beautify("  nope  ", 2), "nope");
        assert_eq!(uglify("  nope  "), "nope");
        assert_eq!(escape("  nope  "), "nope");
        assert_eq!(unescape("  \\q  "), "\\q");
    }
}
