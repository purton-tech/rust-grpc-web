use actix_web::{Error, get, middleware, web, App, HttpRequest, web::Bytes, HttpResponse, HttpServer, Responder, Result};
use hello_world::{HelloReply, HelloRequest, greeter_server};
use crate::hello_world::greeter_server::Greeter;
use base64;
use prost::Message;
use async_trait::async_trait;

pub mod hello_world {
    include!(concat!(env!("OUT_DIR"), concat!("/helloworld.rs")));
}

struct GreeterImpl;

#[async_trait]
impl greeter_server::Greeter for GreeterImpl {
    async fn say_hello(&self, request: HelloRequest) -> Result<HelloReply> {
        Ok(HelloReply {
            message: String::from(format!("Boom {}", request.name))
        })
    }
}

async fn greeter_say_hello(body: Bytes, greeter: web::Data<Box<dyn greeter_server::Greeter>>) -> impl Responder {
    
    let buffer = base64::decode(body).unwrap();
    let s = HelloRequest::decode(buffer.as_ref()).unwrap();

    let reply = greeter.into_inner().say_hello(s).await.unwrap();

    let mut proto_buffer: Vec<u8> = Vec::new();
    reply.encode(&mut proto_buffer).unwrap();
    base64::encode(proto_buffer)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .data::<Box<dyn greeter_server::Greeter>>(Box::new(GreeterImpl {}))
            .route("/helloworld/Greeter/SayHello", web::post().to(greeter_say_hello))
            .wrap(middleware::Logger::default())
    })
    .bind("0.0.0.0:8080")?
    .workers(1)
    .run()
    .await
}