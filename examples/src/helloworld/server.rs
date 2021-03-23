use actix_web::{middleware, web, App, HttpServer, Result};
use hello_world::{HelloReply, HelloRequest, greeter_server};
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .data::<Box<dyn greeter_server::Greeter>>(Box::new(GreeterImpl {}))
            .route("/helloworld/Greeter/SayHello", web::post().to(greeter_server::say_hello))
            .wrap(middleware::Logger::default())
    })
    .bind("0.0.0.0:8080")?
    .workers(1)
    .run()
    .await
}