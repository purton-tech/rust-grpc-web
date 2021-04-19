fn main() {
    rust_grpc_web::configure()
        .build_client(true)
        .build_server(false)
        .compile(&["../protos/quotes.proto"], &["../protos"])
        .unwrap();
}
