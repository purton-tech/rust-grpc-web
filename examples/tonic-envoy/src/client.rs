pub mod quotes {
    tonic::include_proto!("quotes");
}

use quotes::quote_service_client::QuoteServiceClient;
use quotes::Quote;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = QuoteServiceClient::connect("http://[::1]:10000").await?;

    Ok(())
}
