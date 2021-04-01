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

#[derive(Debug)]
pub struct MyQuote {
    receiver: Arc<tokio::sync::mpsc::Receiver<Quote>>,
    sender: Arc<tokio::sync::mpsc::Sender<Quote>>,
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

    async fn add_quote(&self, request: Request<Quote>) -> Result<Response<Quote>, Status> {
        let quote = request.into_inner();

        self.sender.send(quote.clone()).await.unwrap();

        Ok(Response::new(quote))
    }

    async fn query_quotes(
        &self,
        request: Request<QuoteRequest>,
    ) -> Result<Response<Self::QueryQuotesStream>, Status> {
        println!("query_quotes = {:?}", request);

        let (tx, rx) = mpsc::channel(4);
        let receiver = self.receiver.clone();

        tokio::spawn(async move {
            //while let Some(quote) = receiver.recv().await {
            //    println!("GOT = {:?}", quote);
            //    tx.send(Ok(quote.clone())).await.unwrap();
            //}

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

    let (tx, rx) = mpsc::channel(4);

    let quoter = MyQuote {
        receiver: Arc::new(rx),
        sender: Arc::new(tx),
    };

    Server::builder()
        .add_service(QuoteServiceServer::new(quoter))
        .serve(addr)
        .await?;

    Ok(())
}
