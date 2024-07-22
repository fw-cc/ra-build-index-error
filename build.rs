use protobuf_codegen::Codegen;

fn main() {
    Codegen::new()
        .include("src/protos")
        .inputs(["src/protos/example.proto"])
        .cargo_out_dir("protos")
        .run_from_script();
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=src/protos");
}
