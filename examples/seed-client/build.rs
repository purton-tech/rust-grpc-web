fn main() {

    rust_grpc_web::configure()
        .build_client(true)
        .build_server(false)
        .compile(&["../helloworld/proto/helloworld.proto"], &["../helloworld/proto/"])
        .unwrap();

}