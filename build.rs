extern crate prost_build;

fn main() {
    prost_build::compile_protos(&["src/app/geodata/geodata.proto"], &["src/app/geodata/"]).unwrap();
}
