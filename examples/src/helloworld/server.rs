use actix_web::{Error, get, middleware, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use hello_world::{HelloReply, HelloRequest};
use base64;
use prost::Message;

pub mod hello_world {
    include!(concat!(env!("OUT_DIR"), concat!("/helloworld.rs")));
}

// curl -H "Content-Type: application/json" -d '{"name":"xyz"}' http://localhost:8080/helloworld/Greeter/SayHello
async fn greeter_say_hello(hello_request: HttpRequest) -> impl Responder {
    let reply = HelloReply {
        message: String::from("Hello world!")
    };
    let mut proto_buffer: Vec<u8> = Vec::new();
    reply.encode(&mut proto_buffer).unwrap();
    base64::encode(proto_buffer)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/helloworld/Greeter/SayHello", web::post().to(greeter_say_hello))
    })
    .bind("0.0.0.0:8080")?
    .workers(1)
    .run()
    .await
}