use hello_world::{HelloRequest, greeter_client};

pub mod hello_world {
    include!(concat!(env!("OUT_DIR"), concat!("/helloworld.rs")));
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let client = greeter_client::Greeter::new(String::from("http://localhost:8080"));
    
    let req = HelloRequest {
        name: String::from("Purton")
    };

    let res = client.say_hello(req).await?;

    println!("{:#?}", res);
    Ok(())
}