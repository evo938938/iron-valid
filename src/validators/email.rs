use regex::Regex;

use params::{Map, Value};

pub fn validate_email(values: &Map, field: &[&str]) -> Result<Option<Value>, String> {
    lazy_static! {
        static ref EMAIL_REGEX: Regex = Regex::new(r".+@.+\..+").unwrap();
    }

    match values.find(field) {
        Some(&Value::String(ref value)) => {
            if value.is_empty() {
                // Allow empty values
                return Ok(None);
            }
            if EMAIL_REGEX.is_match(value) {
                // Allow valid emails
                return Ok(None);
            }
            Err(format!("The {} field must contain a valid email address.",
                        field.last()
                            .unwrap()
                            .to_lowercase()
                            .replace("_", " ")))
        }
        None => {
            // Allow empty values
            Ok(None)
        }
        _ => {
            Err(format!("The {} field must contain a valid email address.",
                        field.last()
                            .unwrap()
                            .to_lowercase()
                            .replace("_", " ")))
        }
    }
}
