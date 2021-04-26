use yew::prelude::*;
pub mod quotes {
    include!(concat!(env!("OUT_DIR"), concat!("/quotes.rs")));
}
use yewtil::future::LinkFuture;
use quotes::{quote_service_client, HelloRequest, HelloReply};

enum Msg {
    Call,
    ReceiveResponse(Result<HelloReply, Box<dyn std::error::Error>>)
}

struct Model {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    value: String,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: String::from(""),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Call => {
                let grpc_client = quote_service_client::QuoteService::new(
                    String::from("http://localhost:8080"));
                self.link.send_future(async move {
                    Msg::ReceiveResponse(grpc_client.say_hello(HelloRequest {
                        name: String::from("Yew")
                    }).await)
                });
                false
            }
            Msg::ReceiveResponse(Ok(result)) => {
                self.value = result.message;
                true
            }
            Msg::ReceiveResponse(Err(_error)) => {
                self.value = String::from("Error");
                true
            }

        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <button onclick=self.link.callback(|_| Msg::Call)>{ "Call Say Hello" }</button>
                <p>{ &self.value }</p>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}