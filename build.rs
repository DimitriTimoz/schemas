use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use build_helpers::parse_file::read_csv_schema;

pub mod build_helpers;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    println!("{}", out_dir);
    let dest_path = Path::new(&out_dir).join("properties.rs");
    let mut to_write = read_csv_schema();
    let mut file = File::create(&dest_path).unwrap();
    let properties = to_write.write_properties_structs();
    
    // Write properties in file
    file.write_all(properties.as_bytes()).unwrap();
    
}
