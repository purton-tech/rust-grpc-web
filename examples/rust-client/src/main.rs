use quotes::{quote_service_client, HelloRequest, CurrenciesRequest};
use tokio_tungstenite::{
    connect_async, tungstenite::protocol::Message
};
use futures_util::{SinkExt, StreamExt};
pub mod quotes {
    include!(concat!(env!("OUT_DIR"), concat!("/quotes.rs")));
}
use http::Request;

async fn run_test() -> tokio_tungstenite::tungstenite::Result<()> {
    
    let headers = "content-type: application/grpc-web+proto\r\nx-grpc-web: 1\r\n";
    let initial_msg = Message::binary(headers.as_bytes());

    let bytes: Vec<u8> = vec!(0,0,0,0,0,0);
    let bytes_msg = Message::binary(bytes);

    let bytes: Vec<u8> = vec!(1);
    let one_msg = Message::binary(bytes);
    
    let req: Request<()> = Request::builder()
        .uri("ws://localhost:8080/quotes.QuoteService/Subscribe")
        .header("Sec-Websocket-Protocol", "grpc-websockets")
        .body(())
        .unwrap();

    let (mut ws_stream, _) = connect_async(req).await?;
    ws_stream.send(initial_msg).await?;
    ws_stream.send(bytes_msg).await?;
    ws_stream.send(one_msg).await?;
    while let Some(msg) = ws_stream.next().await {
        let msg = msg?;
        if msg.is_text() || msg.is_binary() {
            dbg!(msg);
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
