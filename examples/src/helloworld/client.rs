use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let resp = client.post("http://localhost:8080/helloworld/Greeter/SayHello")
        .body(b"the exact body that is sent")
        .send()
        .await?;
    println!("{:#?}", resp.text().await?);
    Ok(())
}