fn main() {
    
    rust_grpc_web::configure()
        .compile(&["../protos/quotes.proto"], &["../protos/"])
        .unwrap();
}