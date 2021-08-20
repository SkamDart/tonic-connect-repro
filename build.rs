fn main() {
    tonic_build::compile_protos("proto/org/connect/v1/connect.proto").unwrap();
}
