// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]
use quotes::{SubscribeReply, SubscribeRequest};
pub mod quotes {
    include!(concat!(env!("OUT_DIR"), concat!("/quotes.rs")));
}
use prost::Message;

use seed::{prelude::*, *};
use std::rc::Rc;

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    Model {
        price: String::from(""),
        quote_stream: create_quote_stream(orders)
    }
}

// ------ ------
//     Model
// ------ ------

pub struct Model {
    price: String,
    quote_stream: WebSocket
}

// ------ ------
//    Update
// ------ ------

pub enum Msg {
    QuoteStreamOpened,
    QuoteReceived(SubscribeReply),
    QuoteStreamClosed(CloseEvent),
    QuoteStreamFailed,
}

fn update(msg: Msg, mut model: &mut Model, _orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::QuoteReceived(quote) => {
            model.price = quote.key;
        }
        Msg::QuoteStreamOpened => {
            initialise_stream(&mut model.quote_stream);
        }
        Msg::QuoteStreamClosed(close_event) => {
            log!("==================");
            log!("Quote stream connection was closed:");
            log!("Clean:", close_event.was_clean());
            log!("Code:", close_event.code());
            log!("Reason:", close_event.reason());
            log!("==================");
        }
        Msg::QuoteStreamFailed => {
            log!("Quote stream failed");
        }
    }
}

fn websocket_frame_request<T: prost::Message>(request: T) -> Vec<u8> {
    let mut proto_buffer: Vec<u8> = Vec::new();
    request.encode(&mut proto_buffer).unwrap();
    let mut frame: Vec<u8> = vec!(0,0);
    frame.append(&mut (proto_buffer.len() as u32).to_be_bytes().to_vec());
    frame.append(&mut proto_buffer);

    frame
}

fn initialise_stream(web_socket: &mut WebSocket) {
    log!("Sending Headers");
    let headers = "content-type: application/grpc-web+proto\r\nx-grpc-web: 1\r\n";
    web_socket.send_bytes(headers.as_bytes()).unwrap();
    log!("WebSocket connection is open now");

    // Send frame
    let request = SubscribeRequest {};
    let frame = websocket_frame_request(request);
    web_socket.send_bytes(&frame).unwrap();

    // Send finish
    let bytes: Vec<u8> = vec!(1);
    web_socket.send_bytes(&bytes).unwrap();
}

fn create_quote_stream(orders: &impl Orders<Msg>) -> WebSocket {
    let msg_sender = orders.msg_sender();

    WebSocket::builder("ws://localhost:8080/quotes.QuoteService/Subscribe", orders)
        .on_open(|| Msg::QuoteStreamOpened)
        .protocols(&["grpc-websockets"])
        .on_message(move |msg| decode_message(msg, msg_sender))
        .on_close(Msg::QuoteStreamClosed)
        .on_error(|| Msg::QuoteStreamFailed)
        .build_and_open()
        .unwrap()
}

fn decode_message(message: WebSocketMessage, msg_sender: Rc<dyn Fn(Option<Msg>)>) {
    spawn_local(async move {
        let bytes = message
            .bytes()
            .await
            .expect("WebsocketError on binary data");


        if let Ok(decoded) = SubscribeReply::decode(bytes.as_ref()) {
            log!("Msg : ", decoded);
            msg_sender(Some(Msg::QuoteReceived(decoded)));
        }

    });
}

// ------ ------
//     View
// ------ ------

// (Remove the line below once your `Model` become more complex.)
#[allow(clippy::trivially_copy_pass_by_ref)]
// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    div!
    [
        attrs!{
            At::Id => "ticker", 
        },
        input![
            attrs!{
                At::Type => "text", 
                At::Value => "1", 
            },
        ],
        select![
            option!["BTC"],
            option!["ETH"],
        ],
        " = ",
        input![
            attrs!{
                At::Type => "text", 
                At::Value => model.price,
            },
        ],
    ]
}

// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
