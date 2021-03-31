fn main() {
    rust_grpc_web::configure()
        .build_client(true)
        .build_server(true)
        .compile(&["../protos/helloworld.proto"], &["../protos"])
        .unwrap();
}
