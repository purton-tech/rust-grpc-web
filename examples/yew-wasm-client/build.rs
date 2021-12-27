fn main() {
    
    rust_grpc_web::configure()
        .support_streaming(false)
        .compile(&["../protos/quotes.proto"], &["../protos/"])
        .unwrap();
}