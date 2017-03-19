use params::{Map, Value};

pub fn validate_required_without(values: &Map,
                                 field: &[&str],
                                 others: &[Vec<&str>])
                                 -> Result<Option<Value>, String> {
    let mut required = false;
    for other in others {
        if required {
            break;
        }
        required = match values.find(other) {
            None |
            Some(&Value::Null) => true,
            Some(&Value::String(ref value)) if value.is_empty() => true,
            Some(&Value::Array(ref value)) if value.is_empty() => true,
            Some(&Value::Map(ref value)) if value.is_empty() => true,
            _ => false,
        };
    }

    if required {
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
    } else {
        Ok(None)
    }
}
