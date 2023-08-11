use std::{env, fs::{File, self, OpenOptions}, io::{Write, self}, path::Path};
use writer::ToWrite;
use parse_file::read_schema;

mod parse_file;
mod writer;

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap_or(String::from("generated"));
    std::fs::remove_dir_all(&out_dir).unwrap();
    copy_dir_all("base", &out_dir).unwrap();

    let mut table = read_schema();

    // Write properties in file
    let (props, classes, features, types) = ToWrite::write_files(&mut table);

    let mut file = File::create(Path::new(&out_dir).join("src/properties_gen.rs")).unwrap();
    file.write_all(props.as_bytes()).unwrap();

    let mut file = File::create(Path::new(&out_dir).join("src/classes_gen.rs")).unwrap();
    file.write_all(classes.as_bytes()).unwrap();

    let mut file = OpenOptions::new().write(true).append(true).open(Path::new(&out_dir).join("Cargo.toml")).unwrap();
    file.write_all(features.as_bytes()).unwrap();
    file.write_all(b"\n").unwrap();

    let mut file = File::create(Path::new(&out_dir).join("src/types_gen.rs")).unwrap();
    file.write_all(types.as_bytes()).unwrap();
}
