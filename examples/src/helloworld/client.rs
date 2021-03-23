use hello_world::{HelloReply, HelloRequest};
use base64;
use prost::Message;

pub mod hello_world {
    include!(concat!(env!("OUT_DIR"), concat!("/helloworld.rs")));
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let resp = client.post("http://localhost:8080/helloworld/Greeter/SayHello")
        .body("the exact body that is sent")
        .send()
        .await?
        .text()
        .await?;

    let buffer = base64::decode(resp).unwrap();

    let s = HelloReply::decode(buffer.as_ref()).unwrap();

    println!("{:#?}", s);
    Ok(())
}