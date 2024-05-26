use protoc_rust;

fn main() {
    protoc_rust::Codegen::new()
        .out_dir("src")
        .inputs(&["protos/mahimahi.proto"])
        .include("protos")
        .run()
        .expect("Running protoc failed.");
}