// build.rs

use std::env;
use std::fs::{read_to_string, File};
use std::io::{BufWriter, Write};
use std::path::Path;

fn main() {
    let out_dir_env = env::var_os("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir_env);

    protobuf_codegen::Codegen::new()
        .out_dir(out_dir)
        .inputs(&["src/proto/events.proto"])
        .include("src/proto/")
        .run()
        .expect("Codegen failed.");
    // Resolve the path to the generated file.
    let path = out_dir.join("events.rs");
    // Read the generated code to a string.
    let code = read_to_string(&path).expect("Failed to read generated file");
    // Write filtered lines to the same file.
    let mut writer = BufWriter::new(File::create(path).unwrap());
    for line in code.lines() {
        if !line.starts_with("//!") && !line.starts_with("#!") {
            writer.write_all(line.as_bytes()).unwrap();
            writer.write_all(&[b'\n']).unwrap();
        }
    }


    protobuf_codegen::Codegen::new()
        .out_dir(out_dir)
        .inputs(&["src/proto/ingestion.proto"])
        .include("src/proto/")
        .run()
        .expect("Codegen failed.");
    // Resolve the path to the generated file.
    let path = out_dir.join("ingestion.rs");
    // Read the generated code to a string.
    let code = read_to_string(&path).expect("Failed to read generated file");
    // Write filtered lines to the same file.
    let mut writer = BufWriter::new(File::create(path).unwrap());
    for line in code.lines() {
        if !line.starts_with("//!") && !line.starts_with("#!") {
            writer.write_all(line.as_bytes()).unwrap();
            writer.write_all(&[b'\n']).unwrap();
        }
    }
}
