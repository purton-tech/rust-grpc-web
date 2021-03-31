use tonic::{transport::Server, Request, Response, Status};
use tokio::sync::mpsc;
use futures::{Stream, StreamExt};
use std::pin::Pin;
use std::sync::Arc;

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

    async fn get_quote(
        &self,
        request: Request<QuoteRequest>, 
    ) -> Result<Response<Quote>, Status> { 
        unimplemented!() 
    }
    async fn query_quotes(
        &self,
        request: Request<QuoteRequest>,
    ) -> Result<Response<Self::QueryQuotesStream>, Status>  {
        unimplemented!()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let quoter = MyQuote {
        quotes: Arc::new(Vec::new())
    };

    Server::builder()
        .add_service(QuoteServiceServer::new(quoter))
        .serve(addr)
        .await?;

    Ok(())
}
