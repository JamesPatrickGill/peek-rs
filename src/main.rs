mod file_utils;
mod generic_collection;
mod output;
mod parser;

use clap::{App, Arg};
use file_utils::get_file_type;
use output::generate_output;
use parser::parse;

fn main() {
    let matches = App::new("peekrs")
        .version("0.0.1")
        .about("Takes a peek at your data files!")
        .author("JamesPatrickGill")
        .arg(Arg::with_name("file").help("The file to peek").index(1))
        .get_matches();

    let data_content = if matches.is_present("file") {
        let file_path = matches.value_of("file").unwrap();
        let file_type = get_file_type(file_path).unwrap();
        parse(file_path, file_type)
    } else {
        generic_collection::PeekValue::Null
    };

    println!("{}", generate_output(data_content));
}
