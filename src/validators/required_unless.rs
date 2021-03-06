use params::{Map, Value};

pub fn validate_required_unless(values: &Map,
                                field: &[&str],
                                other: &[&str],
                                condition: &Value)
                                -> Result<Option<Value>, String> {
    match values.find(other) {
        Some(value) if *value == *condition => Ok(None),
        _ => {
            match values.find(field) {
                Some(&Value::String(ref value)) if value.is_empty() => {
                    Err(format!("The {} field is required.",
                                field.last()
                                    .unwrap()
                                    .to_lowercase()
                                    .replace("_", " ")))
                }
                Some(&Value::Array(ref value)) if value.is_empty() => {
                    Err(format!("The {} field is required.",
                                field.last()
                                    .unwrap()
                                    .to_lowercase()
                                    .replace("_", " ")))
                }
                Some(&Value::Map(ref value)) if value.is_empty() => {
                    Err(format!("The {} field is required.",
                                field.last()
                                    .unwrap()
                                    .to_lowercase()
                                    .replace("_", " ")))
                }
                Some(&Value::Null) |
                None => {
                    Err(format!("The {} field is required.",
                                field.last()
                                    .unwrap()
                                    .to_lowercase()
                                    .replace("_", " ")))
                }
                _ => Ok(None),
            }
        }
    }
}
