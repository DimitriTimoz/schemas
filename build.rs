use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use build_helpers::parse_file::Table;
use build_helpers::writer::ToWrite;

use crate::build_helpers::parse_file::read_schema;

pub mod build_helpers;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let mut table = read_schema();

    let (props, calss) = ToWrite::write_files(&table);
    let mut file = File::create(&Path::new(&out_dir).join("properties.rs")).unwrap();
    file.write_all(props.as_bytes()).unwrap();
    let mut file = File::create(&Path::new(&out_dir).join("types.rs")).unwrap();
    file.write_all("".to_string().as_bytes()).unwrap();

    // Write properties in file
}
