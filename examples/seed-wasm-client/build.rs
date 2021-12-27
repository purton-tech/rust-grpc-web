fn main() {
    rust_grpc_web::configure()
        .build_websys_client(true)
        .support_streaming(true)
        .compile(&["../protos/quotes.proto"], &["../protos/"])
        .unwrap();

}