mod json_parser;
mod yaml_parser;

use crate::{file_utils::FileType, generic_collection::PeekValue};

use self::{json_parser::parse_json, yaml_parser::parse_yaml};

pub fn parse(file_path: &str, file_type: FileType) -> PeekValue {
    match file_type {
        FileType::Json => parse_json(file_path),
        FileType::Yaml => parse_yaml(file_path),
    }
}
