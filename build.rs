use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use build_helpers::parse_file::read_csv_schema;

pub mod build_helpers;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let mut to_write = read_csv_schema();

    let props_path: std::path::PathBuf = Path::new(&out_dir).join("properties.rs");
    let mut file = File::create(props_path).unwrap();
    let properties = to_write.write_properties_structs();
    file.write_all(properties.as_bytes()).unwrap();

    let types_path: std::path::PathBuf = Path::new(&out_dir).join("types.rs");
    let mut file = File::create(types_path).unwrap();
    let types = to_write.write_type_structs();
    file.write_all(types.as_bytes()).unwrap();

    // Write properties in file
}
