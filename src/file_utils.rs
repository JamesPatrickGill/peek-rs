use std::ffi::OsStr;
use std::path::Path;

fn get_extension_from_filename(filename: &str) -> Option<&str> {
    Path::new(filename).extension().and_then(OsStr::to_str)
}

pub enum FileType {
    Json,
    Yaml,
}

pub fn get_file_type(filename: &str) -> Result<FileType, &str> {
    match get_extension_from_filename(filename) {
        Some(ext) if ext == "json" => Ok(FileType::Json),
        Some(ext) if ext == "yaml" => Ok(FileType::Yaml),
        Some(ext) if ext == "yml" => Ok(FileType::Yaml),
        Some(_) => Err("This file type is not supported"),
        None => Err("No file extension"),
    }
}
