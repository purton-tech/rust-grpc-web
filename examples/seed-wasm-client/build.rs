fn main() {

    rust_grpc_web::configure()
        .build_client(false)
        .build_server(false)
        .compile(&["../protos/helloworld.proto"], &["../protos/"])
        .unwrap();

}