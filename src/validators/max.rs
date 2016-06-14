use params::{Map, Value};
use std::f64;

pub fn validate_max(values: &Map, field: &str, target: isize) -> Result<Option<Value>, String> {
    match values.find(&[field]) {
        Some(&Value::String(ref value)) => {
            if value.is_empty() {
                // Allow empty values
                return Ok(None);
            }
            assert!(target >= 0);
            if value.len() <= target as usize {
                Ok(None)
            } else {
                Err(format!("The {} field must be no greater than {} characters.",
                            field.to_lowercase().replace("_", " "),
                            target))
            }
        }
        Some(&Value::U64(ref value)) => {
            if target >= 0 && *value <= target as u64 {
                Ok(None)
            } else {
                Err(format!("The {} field must be no greater than {}.",
                            field.to_lowercase().replace("_", " "),
                            target))
            }
        }
        Some(&Value::I64(ref value)) => {
            if *value <= target as i64 {
                Ok(None)
            } else {
                Err(format!("The {} field must be no greater than {}.",
                            field.to_lowercase().replace("_", " "),
                            target))
            }
        }
        Some(&Value::F64(ref value)) => {
            if *value <= target as f64 {
                Ok(None)
            } else {
                Err(format!("The {} field must be no greater than {}.",
                            field.to_lowercase().replace("_", " "),
                            target))
            }
        }
        Some(&Value::Array(ref value)) => {
            if value.is_empty() {
                // Allow empty values
                return Ok(None);
            }
            assert!(target >= 0);
            if value.len() <= target as usize {
                Ok(None)
            } else {
                Err(format!("The {} field must have no greater than {} items.",
                            field.to_lowercase().replace("_", " "),
                            target))
            }
        }
        Some(&Value::Map(ref value)) => {
            if value.is_empty() {
                // Allow empty values
                return Ok(None);
            }
            assert!(target >= 0);
            if value.len() <= target as usize {
                Ok(None)
            } else {
                Err(format!("The {} field must have no greater than {} items.",
                            field.to_lowercase().replace("_", " "),
                            target))
            }
        }
        Some(&Value::File(ref value)) => {
            assert!(target >= 0);
            if value.size() >> 10 <= target as u64 {
                Ok(None)
            } else {
                Err(format!("The {} must be no greater than {} kilobytes.",
                            field.to_lowercase().replace("_", " "),
                            target))
            }
        }
        None => {
            // Allow empty values
            Ok(None)
        }
        _ => {
            Err(format!("The {} field must have no greater than a size of {}.",
                        field.to_lowercase().replace("_", " "),
                        target))
        }
    }
}