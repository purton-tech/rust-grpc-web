use std::env;
use std::path::PathBuf;

fn main() {

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    actix_browser_rpc::configure()
        .build_client(true)
        .build_server(true)
        .file_descriptor_set_path(out_dir.join("helloworld_descriptor.bin"))
        .compile(&["proto/helloworld.proto"], &["proto"])
        .unwrap();

}