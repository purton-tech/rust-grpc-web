use hello_world::{HelloReply, HelloRequest};
use base64;
use prost::Message;

pub mod hello_world {
    include!(concat!(env!("OUT_DIR"), concat!("/helloworld.rs")));
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let req = HelloRequest {
        name: String::from("Purton")
    };
    let mut proto_buffer: Vec<u8> = Vec::new();
    req.encode(&mut proto_buffer).unwrap();
    let base64 = base64::encode(proto_buffer);


    let client = reqwest::Client::new();
    let resp = client.post("http://localhost:8080/helloworld/Greeter/SayHello")
        .body(base64)
        .send()
        .await?
        .text()
        .await?;

    let buffer = base64::decode(resp).unwrap();

    let s = HelloReply::decode(buffer.as_ref()).unwrap();

    println!("{:#?}", s);
    Ok(())
}