use quotes::{quote_service_client, HelloRequest, CurrenciesRequest};
use tokio_tungstenite::{
    connect_async,
};
use futures_util::{SinkExt, StreamExt};
use url::Url;
pub mod quotes {
    include!(concat!(env!("OUT_DIR"), concat!("/quotes.rs")));
}

async fn run_test() -> tokio_tungstenite::tungstenite::Result<()> {
    
    // Try a websocket
    let case_url =
        Url::parse("ws://localhost:8080/quotes.QuoteService/Subscribe")
            .expect("Bad testcase URL");

    let (mut ws_stream, _) = connect_async(case_url).await?;
    while let Some(msg) = ws_stream.next().await {
        let msg = msg?;
        if msg.is_text() || msg.is_binary() {
            ws_stream.send(msg).await?;
        }
    }

    Ok(())
}

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

    run_test().await.expect("It Didn't work");

    Ok(())
}
