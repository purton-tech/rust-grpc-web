use futures::Stream;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::mpsc;
use tonic::{transport::Server, Request, Response, Status};

use quotes::quote_service_server::{QuoteService, QuoteServiceServer};
use quotes::{Quote, QuoteRequest};

pub mod quotes {
    tonic::include_proto!("quotes"); // The string specified here must match the proto package name
}

#[derive(Debug, Default)]
pub struct MyQuote {
    quotes: Arc<Vec<Quote>>,
}

#[tonic::async_trait]
impl QuoteService for MyQuote {
    type QueryQuotesStream =
        Pin<Box<dyn Stream<Item = Result<Quote, Status>> + Send + Sync + 'static>>;

    async fn get_quote(&self, _request: Request<QuoteRequest>) -> Result<Response<Quote>, Status> {
        Ok(Response::new(Quote {
            ticker: "BTC".into(),
            price: 100,
            currency: "GBP".into(),
        }))
    }
    async fn query_quotes(
        &self,
        request: Request<QuoteRequest>,
    ) -> Result<Response<Self::QueryQuotesStream>, Status> {
        println!("query_quotes = {:?}", request);

        let (tx, rx) = mpsc::channel(4);
        let quotes = self.quotes.clone();

        tokio::spawn(async move {
            for quote in &quotes[..] {
                tx.send(Ok(quote.clone())).await.unwrap();
            }

            println!(" /// done sending");
        });

        Ok(Response::new(Box::pin(
            tokio_stream::wrappers::ReceiverStream::new(rx),
        )))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse()?;
    let quoter = MyQuote {
        quotes: Arc::new(Vec::new()),
    };

    Server::builder()
        .add_service(QuoteServiceServer::new(quoter))
        .serve(addr)
        .await?;

    Ok(())
}
