use quotes::{quote_service_client, CurrenciesRequest};

pub mod quotes {
    include!(concat!(env!("OUT_DIR"), concat!("/quotes.rs")));
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = quote_service_client::QuoteService::new(String::from("http://localhost:8080"));

    let req = CurrenciesRequest {};

    let res = client.get_currencies(req).await?;

    println!("{:#?}", res);
    Ok(())
}
