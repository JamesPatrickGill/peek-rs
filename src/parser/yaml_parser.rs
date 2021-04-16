use serde_yaml::Value;

use crate::{
    generic_collection::PeekArray, generic_collection::PeekMap, generic_collection::PeekValue,
};
use std::{collections::HashMap, fs::read_to_string};

fn convert_yaml_number(number: serde_yaml::Number) -> PeekValue {
    if number.is_i64() {
        PeekValue::Integer(number.as_i64().unwrap())
    } else {
        PeekValue::Float(number.as_f64().unwrap())
    }
}

fn convert_yaml_value_to_string(value: serde_yaml::Value) -> String {
    return match value {
        Value::Null => String::from("[null]"),
        Value::String(ser_string) => ser_string,
        Value::Number(ser_num) => ser_num.to_string(),
        Value::Bool(_) => String::from("[boolean]"),
        Value::Sequence(_) => String::from("[array]"),
        Value::Mapping(_) => String::from("[map]"),
    };
}

fn parse_serde_yaml_mapping(map: serde_yaml::Mapping) -> PeekMap {
    let mut peek_map: HashMap<String, PeekValue> = HashMap::new();

    for entry in map.iter() {
        peek_map.insert(
            convert_yaml_value_to_string(entry.0.to_owned()),
            parse_serde_yaml_value(entry.1.to_owned()),
        );
    }

    PeekMap::new(peek_map)
}

fn parse_serde_yaml_sequence(vec: Vec<Value>) -> PeekArray {
    let mut peek_arr: Vec<PeekValue> = Vec::new();

    for entry in vec.iter() {
        peek_arr.push(parse_serde_yaml_value(entry.to_owned()));
    }

    PeekArray::new(peek_arr)
}

fn parse_serde_yaml_value(value: serde_yaml::Value) -> PeekValue {
    return match value {
        Value::Null => PeekValue::Null,
        Value::String(ser_string) => PeekValue::String(ser_string),
        Value::Number(ser_num) => convert_yaml_number(ser_num),
        Value::Bool(ser_boolean) => PeekValue::Bool(ser_boolean),
        Value::Sequence(ser_seq) => PeekValue::Array(parse_serde_yaml_sequence(ser_seq)),
        Value::Mapping(ser_map) => PeekValue::Map(parse_serde_yaml_mapping(ser_map)),
    };
}

pub fn parse_yaml(file_path: &str) -> PeekValue {
    let file_contents = read_to_string(file_path).unwrap();
    let parsed_value: serde_yaml::Value = serde_yaml::from_str(&file_contents).unwrap();

    parse_serde_yaml_value(parsed_value)
}
