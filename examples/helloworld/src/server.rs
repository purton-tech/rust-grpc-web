use actix_web::{middleware, App, HttpServer, HttpRequest, Result, options};
use hello_world::{HelloReply, HelloRequest, greeter_server};
use async_trait::async_trait;

pub mod hello_world {
    include!(concat!(env!("OUT_DIR"), concat!("/helloworld.rs")));
}

struct GreeterImpl;

#[async_trait]
impl greeter_server::Greeter for GreeterImpl {
    async fn say_hello(&self, hello_request: HelloRequest) -> Result<HelloReply> {
        Ok(HelloReply {
            message: String::from(format!("Hello {}", hello_request.name))
        })
    }
}

#[options("/helloworld.Greeter/SayHello")]
async fn option(_req: HttpRequest) -> &'static str {
    "Hello world!"
}

use prost::Message;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let request = HelloRequest {
        name: String::from("World")
    };
    let mut proto_buffer: Vec<u8> = Vec::new();
    request.encode(&mut proto_buffer).unwrap();
    let base64 = base64::encode(proto_buffer);
    dbg!(base64);

    HttpServer::new(|| {

        App::new()
            .wrap(middleware::DefaultHeaders::new()
                .header("Access-Control-Allow-Origin", "*")
                .header("Access-Control-Allow-Headers", "*"))
            .data::<Box<dyn greeter_server::Greeter>>(Box::new(GreeterImpl {}))
            .service(option)
            .configure(greeter_server::routes)
            .wrap(middleware::Logger::default())
    })
    .bind("0.0.0.0:8080")?
    .workers(1)
    .run()
    .await
}