extern crate iron_valid;
extern crate params;

use iron_valid::{Rule, validate};
use params::{Map, Value};
use std::collections::BTreeMap;

#[test]
fn test_not_in_array_valid_string() {
    let mut params = Map::new();
    params.assign("in", Value::String("1".to_owned())).ok();
    params.assign("other",
                  Value::Array(vec![Value::String("2".into()), Value::U64(1)]))
        .ok();

    let mut rules = BTreeMap::new();
    rules.insert("in", vec![Rule::NotInArray("other")]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["in"]).unwrap(),
               &Value::String("1".to_owned()));
}

#[test]
fn test_not_in_array_invalid_string() {
    let mut params = Map::new();
    params.assign("in", Value::String("2".to_owned())).ok();
    params.assign("other",
                  Value::Array(vec![Value::String("2".into()), Value::U64(1)]))
        .ok();

    let mut rules = BTreeMap::new();
    rules.insert("in", vec![Rule::NotInArray("other")]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("in").unwrap(),
               vec!["The in field must not be one of the values in the other field.".to_owned()]);
}

#[test]
fn test_not_in_array_invalid_other_blank() {
    let mut params = Map::new();
    params.assign("in", Value::String("2".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("in", vec![Rule::NotInArray("other")]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("in").unwrap(),
               vec!["The in field must not be one of the values in the other field.".to_owned()]);
}

#[test]
fn test_not_in_array_valid_numeric() {
    let mut params = Map::new();
    params.assign("in", Value::U64(2)).ok();
    params.assign("other",
                  Value::Array(vec![Value::String("2".into()), Value::U64(1)]))
        .ok();

    let mut rules = BTreeMap::new();
    rules.insert("in", vec![Rule::NotInArray("other")]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["in"]).unwrap(), &Value::U64(2));
}

#[test]
fn test_not_in_array_invalid_numeric() {
    let mut params = Map::new();
    params.assign("in", Value::U64(1)).ok();
    params.assign("other",
                  Value::Array(vec![Value::String("2".into()), Value::U64(1)]))
        .ok();

    let mut rules = BTreeMap::new();
    rules.insert("in", vec![Rule::NotInArray("other")]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("in").unwrap(),
               vec!["The in field must not be one of the values in the other field.".to_owned()]);
}

#[test]
fn test_not_in_array_valid_empty() {
    let mut params = Map::new();
    params.assign("in", Value::String("".into())).ok();
    params.assign("other",
                  Value::Array(vec![Value::String("2".into()), Value::U64(1)]))
        .ok();

    let mut rules = BTreeMap::new();
    rules.insert("in", vec![Rule::NotInArray("other")]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["in"]).unwrap(),
               &Value::String("".into()));
}

#[test]
fn test_not_in_array_valid_blank() {
    let mut params = Map::new();
    params.assign("other",
                  Value::Array(vec![Value::String("2".into()), Value::U64(1)]))
        .ok();

    let mut rules = BTreeMap::new();
    rules.insert("in", vec![Rule::NotInArray("other")]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["in"]), None);
}

#[test]
fn test_not_in_array_valid_null() {
    let mut params = Map::new();
    params.assign("in", Value::Null).ok();
    params.assign("other",
                  Value::Array(vec![Value::String("2".into()), Value::U64(1)]))
        .ok();

    let mut rules = BTreeMap::new();
    rules.insert("in", vec![Rule::NotInArray("other")]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["in"]).unwrap(), &Value::Null);
}

#[test]
fn test_not_in_array_valid_nested() {
    let mut test = Map::new();
    test.assign("in", Value::String("1".to_owned())).ok();
    test.assign("other",
                Value::Array(vec![Value::String("2".into()), Value::U64(1)]))
        .ok();
    let mut params = Map::new();
    params.assign("test", Value::Map(test)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("test.in", vec![Rule::NotInArray("test.other")]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["test", "in"]).unwrap(),
               &Value::String("1".to_owned()));
}
