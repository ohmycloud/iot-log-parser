extern crate prost_build;

fn main() {
    prost_build::compile_protos(
        &["proto/message.proto", "proto/kvpair.proto"],
        &["proto/"],
    )
    .unwrap();
}