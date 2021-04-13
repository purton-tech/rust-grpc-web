use quotes::{quote_service_client, HelloRequest, CurrenciesRequest};

pub mod quotes {
    include!(concat!(env!("OUT_DIR"), concat!("/quotes.rs")));
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

    Ok(())
}
