use super::{Method, Service};
use crate::{generate_doc_comments, naive_snake_case};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

/// Generate service for client.
///
/// This takes some `Service` and will generate a `TokenStream` that contains
/// a public module with the generated client.
pub fn generate<T: Service>(
    service: &T,
    proto_path: &str,
    compile_well_known_types: bool,
) -> TokenStream {
    let client_mod = quote::format_ident!("{}_client", naive_snake_case(&service.name()));
    let service_name = quote::format_ident!("{}", service.name());
    let methods = generate_methods(service, proto_path, compile_well_known_types);

    quote! {
        /// Generated client implementations.
        pub mod #client_mod {
            #![allow(unused_variables, dead_code, missing_docs)]
            use base64;
            use prost::Message;
            pub struct #service_name {
                host: String
            }

            impl #service_name {
                #methods

                pub fn new(host: String) -> #service_name {
                    #service_name {
                        host
                    }
                }
            }
        }
    }
}

fn generate_methods<T: Service>(
    service: &T,
    proto_path: &str,
    compile_well_known_types: bool,
) -> TokenStream {
    let mut stream = TokenStream::new();

    for method in service.methods() {

        stream.extend(generate_doc_comments(method.comment()));

        let method = match (method.client_streaming(), method.server_streaming()) {
            (false, false) => generate_unary(service, method, proto_path, compile_well_known_types),
            (false, true) => {
                TokenStream::new()
            }
            (true, false) => {
                TokenStream::new()
            }
            (true, true) => TokenStream::new(),
        };

        stream.extend(method);
    }

    stream
}

fn generate_unary<T: Method, S: Service>(
    service: &S,
    method: &T,
    proto_path: &str,
    compile_well_known_types: bool,
) -> TokenStream {
    let ident = format_ident!("{}", method.name());
    let (request, response) = method.request_response_name(proto_path, compile_well_known_types);
    let url = format!("/{}.{}/{}", service.package(), service.name(), method.identifier());

    quote! {
        pub async fn #ident(
            &self,
            request: #request
        ) -> Result<#response, Box<dyn std::error::Error>> {
            let mut proto_buffer: Vec<u8> = Vec::new();
            request.encode(&mut proto_buffer).unwrap();
            let mut frame: Vec<u8> = Vec::new();
            frame.push(0 as u8);
            frame.append(&mut (proto_buffer.len() as u32).to_be_bytes().to_vec());
            frame.append(&mut proto_buffer);
            let base64 = base64::encode(frame);

            let client = reqwest::Client::new();
            let resp = client.post(format!("{}{}", &self.host, #url))
                .body(base64)
                .send()
                .await?
                .text()
                .await?;

            let buffer = base64::decode(resp)?;
            
            let s = #response::decode(buffer.as_ref())?;
            Ok(s)
        }
    }
}