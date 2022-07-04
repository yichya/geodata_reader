extern crate prost_build;

fn main() {
    prost_build::compile_protos(&["src/geodata/geodata.proto"], &["src/geodata/"]).unwrap();
}
