use crate::generic_collection::{PeekArray, PeekMap, PeekValue};

use colored::*;

enum CustomFormatError {
    InvalidRootObject,
}

fn get_color(depth: u16) -> (u8, u8, u8) {
    let colors = [(10, 151, 183), (49, 196, 195), (146, 224, 210)];
    let index: usize = depth as usize % colors.len();
    return colors[index];
}

fn get_spacing(depth: u16, has_icon: bool) -> String {
    if depth == 0 {
        return String::from("");
    }
    let spacing: String = (0..depth).map(|_| format!("{:>2}", " ")).collect();
    let spacing_with_indicator: String = if has_icon {
        format!("{}{}", spacing, "↳")
    } else {
        spacing
    };
    spacing_with_indicator
}

fn format_string(string: String, depth: u16) -> String {
    format!("{} {}\n", get_spacing(depth, true), string)
}

fn format_float(float: f64, depth: u16) -> String {
    format!("{} {}\n", get_spacing(depth, true), float)
}

fn format_int(int: i64, depth: u16) -> String {
    format!("{} {}\n", get_spacing(depth, true), int)
}

fn format_bool(boolean: bool, depth: u16) -> String {
    format!("{} {}\n", get_spacing(depth, true), boolean)
}

fn format_array(arr: PeekArray, depth: u16) -> String {
    arr.data
        .into_iter()
        .map(|val| format!("{}", format_value(val, depth).ok().unwrap()))
        .collect()
}

fn format_map(pk_map: PeekMap, depth: u16) -> String {
    pk_map
        .entries()
        .into_iter()
        .map(|(k, v)| {
            let value_string = format_value(v, depth + 1).ok().unwrap();
            let col = get_color(depth);
            return format!(
                "{}[{}]\n{}",
                get_spacing(depth, false),
                k.bold().truecolor(col.0, col.1, col.2),
                value_string
            );
        })
        .collect()
}

fn format_value(value: PeekValue, depth: u16) -> Result<String, CustomFormatError> {
    return match value {
        PeekValue::String(string) => Ok(format_string(string, depth)),
        PeekValue::Float(float) => Ok(format_float(float, depth)),
        PeekValue::Integer(int) => Ok(format_int(int, depth)),
        PeekValue::Bool(boolean) => Ok(format_bool(boolean, depth)),
        PeekValue::Array(arr) => Ok(format_array(arr, depth)),
        PeekValue::Map(map) => Ok(format_map(map, depth)),
        _ => Err(CustomFormatError::InvalidRootObject),
    };
}

pub fn generate_output(value: PeekValue) -> String {
    format_value(value, 0).ok().unwrap()
}
