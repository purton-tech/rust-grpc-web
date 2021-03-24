use std::env;
use std::path::PathBuf;

fn main() {

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    rust_grpc_web::configure()
        .build_client(true)
        .build_server(true)
        .compile(&["proto/helloworld.proto"], &["proto"])
        .unwrap();

}