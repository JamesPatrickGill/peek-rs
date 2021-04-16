use serde_json::Value;

use crate::{
    generic_collection::PeekArray, generic_collection::PeekMap, generic_collection::PeekValue,
};
use std::{collections::HashMap, fs::read_to_string};

fn convert_json_number(number: serde_json::Number) -> PeekValue {
    if number.is_i64() {
        PeekValue::Integer(number.as_i64().unwrap())
    } else {
        PeekValue::Float(number.as_f64().unwrap())
    }
}

fn parse_serde_json_map(map: serde_json::Map<String, Value>) -> PeekMap {
    let mut peek_map: HashMap<String, PeekValue> = HashMap::new();

    for entry in map.iter() {
        peek_map.insert(
            entry.0.to_owned(),
            parse_serde_json_value(entry.1.to_owned()),
        );
    }

    PeekMap::new(peek_map)
}

fn parse_serde_json_array(vec: Vec<Value>) -> PeekArray {
    let mut peek_arr: Vec<PeekValue> = Vec::new();

    for entry in vec.iter() {
        peek_arr.push(parse_serde_json_value(entry.to_owned()));
    }

    PeekArray::new(peek_arr)
}

fn parse_serde_json_value(value: serde_json::Value) -> PeekValue {
    return match value {
        Value::Null => PeekValue::Null,
        Value::String(ser_string) => PeekValue::String(ser_string),
        Value::Number(ser_num) => convert_json_number(ser_num),
        Value::Bool(ser_boolean) => PeekValue::Bool(ser_boolean),
        Value::Array(ser_vec) => PeekValue::Array(parse_serde_json_array(ser_vec)),
        Value::Object(ser_map) => PeekValue::Map(parse_serde_json_map(ser_map)),
    };
}
pub fn parse_json(file_path: &str) -> PeekValue {
    let file_contents = read_to_string(file_path).unwrap();
    let parsed_value: serde_json::Value = serde_json::from_str(&file_contents).unwrap();

    parse_serde_json_value(parsed_value)
}
