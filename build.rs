use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use build_helpers::parse_file::read_csv_schema;

mod build_helpers;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("gen.rs");
    read_csv_schema();
    let mut f = File::create(&dest_path).unwrap();

    // Lire le fichier ici et écrire la sortie à `f`.
    // ...
}
