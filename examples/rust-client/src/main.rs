use quotes::{quote_service_client, HelloRequest, CurrenciesRequest, SubscribeRequest};
pub mod quotes {
    include!(concat!(env!("OUT_DIR"), concat!("/quotes.rs")));
}
use futures_util::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = quote_service_client::QuoteService::new(String::from("http://localhost:8080"));

    let req = HelloRequest {
        name: String::from("World!"),
    };

    let res = client.say_hello(req).await?;

    println!("{:#?}", res);
    
    let req = CurrenciesRequest {};

    let res = client.get_currencies(req).await?;

    println!("{:#?}", res);

    let sub_req = SubscribeRequest {};

    let mut stream = client.subscribe(sub_req).await?;

    while let Some(msg) = stream.next().await {
        dbg!(msg);
    }

    Ok(())
}
