// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]
use quotes::{HelloRequest, CurrenciesRequest, SubscribeReply, SubscribeRequest};
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
        web_socket: create_websocket(orders),
        web_socket_reconnector: None,
    }
}

// ------ ------
//     Model
// ------ ------

pub struct Model {
    price: String,
    web_socket: WebSocket,
    web_socket_reconnector: Option<StreamHandle>
}

// ------ ------
//    Update
// ------ ------


pub enum Msg {
    WebSocketOpened,
    CloseWebSocket,
    BinaryMessageReceived(Vec<u8>),
    WebSocketClosed(CloseEvent),
    WebSocketFailed,
    ReconnectWebSocket(usize),
    InputTextChanged(String),
    InputBinaryChanged(String),
}
fn websocket_frame_request<T: prost::Message>(request: T) -> Vec<u8> {
    let mut proto_buffer: Vec<u8> = Vec::new();
    request.encode(&mut proto_buffer).unwrap();
    let mut frame: Vec<u8> = vec!(0,0);
    frame.append(&mut (proto_buffer.len() as u32).to_be_bytes().to_vec());
    frame.append(&mut proto_buffer);

    frame
}

fn update(msg: Msg, mut model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::BinaryMessageReceived(bytes) => {

            if let Ok(decoded) = SubscribeReply::decode(bytes.as_ref()) {
                log!("Msg : ", decoded);
                model.price = decoded.key;
            }
        }
        Msg::WebSocketOpened => {
            log!("Sending Headers");
            model.web_socket_reconnector = None;
            let headers = "content-type: application/grpc-web+proto\r\nx-grpc-web: 1\r\n";
            model.web_socket.send_bytes(headers.as_bytes()).unwrap();
            log!("WebSocket connection is open now");

            // Send frame
            let request = SubscribeRequest {};
            let frame = websocket_frame_request(request);
            model.web_socket.send_bytes(&frame);

            // Send finish
            let bytes: Vec<u8> = vec!(1);
            model.web_socket.send_bytes(&bytes);
            
        }
        Msg::CloseWebSocket => {
            model.web_socket_reconnector = None;
            model
                .web_socket
                .close(None, Some("user clicked Close button"))
                .unwrap();
        }
        Msg::WebSocketClosed(close_event) => {
            log!("==================");
            log!("WebSocket connection was closed:");
            log!("Clean:", close_event.was_clean());
            log!("Code:", close_event.code());
            log!("Reason:", close_event.reason());
            log!("==================");

            // Chrome doesn't invoke `on_error` when the connection is lost.
            if !close_event.was_clean() && model.web_socket_reconnector.is_none() {
                model.web_socket_reconnector = Some(
                    orders.stream_with_handle(streams::backoff(None, Msg::ReconnectWebSocket)),
                );
            }
        }
        Msg::WebSocketFailed => {
            log!("WebSocket failed2");
            if model.web_socket_reconnector.is_none() {
                model.web_socket_reconnector = Some(
                    orders.stream_with_handle(streams::backoff(None, Msg::ReconnectWebSocket)),
                );
            }
        }
        Msg::ReconnectWebSocket(retries) => {
            //log!("Reconnect attempt:", retries);
            //model.web_socket = create_websocket(orders);
        }
        Msg::InputTextChanged(text) => {
        }
        Msg::InputBinaryChanged(text) => {
        }
    }
}

fn create_websocket(orders: &impl Orders<Msg>) -> WebSocket {
    let msg_sender = orders.msg_sender();

    WebSocket::builder("ws://localhost:8080/quotes.QuoteService/Subscribe", orders)
        .on_open(|| Msg::WebSocketOpened)
        .protocols(&["grpc-websockets"])
        .on_message(move |msg| decode_message(msg, msg_sender))
        .on_close(Msg::WebSocketClosed)
        .on_error(|| Msg::WebSocketFailed)
        .build_and_open()
        .unwrap()
}

fn decode_message(message: WebSocketMessage, msg_sender: Rc<dyn Fn(Option<Msg>)>) {
    if message.contains_text() {

        //msg_sender(Some(Msg::TextMessageReceived(msg)));
    } else {
        spawn_local(async move {
            let bytes = message
                .bytes()
                .await
                .expect("WebsocketError on binary data");

            msg_sender(Some(Msg::BinaryMessageReceived(bytes)));
        });
    }
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
