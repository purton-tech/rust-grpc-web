use super::{Method, Service};
use crate::{generate_doc_comment, generate_doc_comments, naive_snake_case};
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

/// Generate service for Server.
///
/// This takes some `Service` and will generate a `TokenStream` that contains
/// a public module containing the server service and handler trait.
pub fn generate<T: Service>(
    service: &T,
    _emit_package: bool,
    proto_path: &str,
    compile_well_known_types: bool,
) -> TokenStream {
    let server_trait = quote::format_ident!("{}", service.name());

    let marshall_methods = generate_marshalling_methods(
        service,
        proto_path,
        compile_well_known_types,
    );

    let routes = generate_routes(
        service,
    );

    let server_mod = quote::format_ident!("{}_server", naive_snake_case(&service.name()));
    let generated_trait = generate_trait(
        service,
        proto_path,
        compile_well_known_types,
        server_trait.clone(),
    );

    quote! {
        /// Generated server implementations.
        pub mod #server_mod {
            #![allow(unused_variables, dead_code, missing_docs)]
            use async_trait::async_trait;
            use actix_web::{web, web::Bytes, Responder, Result};
            use base64;
            use prost::Message;

            #generated_trait
            
            #marshall_methods

            pub fn routes(cfg: &mut web::ServiceConfig) {
                #routes
            }
        }
    }
}

fn generate_trait<T: Service>(
    service: &T,
    proto_path: &str,
    compile_well_known_types: bool,
    server_trait: Ident,
) -> TokenStream {
    let methods = generate_trait_methods(service, proto_path, compile_well_known_types);
    let trait_doc = generate_doc_comment(&format!(
        "Generated trait containing gRPC methods that should be implemented for use with {}Server.",
        service.name()
    ));

    quote! {
        #trait_doc
        #[async_trait]
        pub trait #server_trait : Send + Sync + 'static {
            #methods
        }
    }
}

fn generate_trait_methods<T: Service>(
    service: &T,
    proto_path: &str,
    compile_well_known_types: bool,
) -> TokenStream {
    let mut stream = TokenStream::new();

    for method in service.methods() {
        let name = quote::format_ident!("{}", method.name());

        let (req_message, res_message) =
            method.request_response_name(proto_path, compile_well_known_types);

        let method_doc = generate_doc_comments(method.comment());

        let method = match (method.client_streaming(), method.server_streaming()) {
            (false, false) => {
                quote! {
                    #method_doc
                    async fn #name(&self, request: #req_message)
                        -> Result<#res_message>;
                }
            }
            (true, false) => {
                TokenStream::new()
            }
            (false, true) => {
                TokenStream::new()
            }
            (true, true) => {
                TokenStream::new()
            }
        };

        stream.extend(method);
    }

    stream
}

fn generate_routes<T: Service>(
    service: &T,
    ) -> TokenStream {
    let mut stream = TokenStream::new();

    for method in service.methods() {
        let name = quote::format_ident!("{}", method.name());

        let url = format!("/{}.{}/{}", service.package(), service.name(), method.identifier());

        let method = match (method.client_streaming(), method.server_streaming()) {
            (false, false) => {
                quote! {
                    cfg.service(web::resource(#url).route(web::post().to(#name)));
                }
            }
            (true, false) => {
                TokenStream::new()
            }
            (false, true) => {
                TokenStream::new()
            }
            (true, true) => {
                TokenStream::new()
            }
        };

        stream.extend(method);
    }

    stream
}

fn generate_marshalling_methods<T: Service>(
    service: &T,
    proto_path: &str,
    compile_well_known_types: bool,
    ) -> TokenStream {
    let mut stream = TokenStream::new();

    for method in service.methods() {
        let name = quote::format_ident!("{}", method.name());

        let (req_message, _) =
            method.request_response_name(proto_path, compile_well_known_types);

        let method = match (method.client_streaming(), method.server_streaming()) {
            (false, false) => {
                quote! {
                    pub async fn #name(body: Bytes, greeter: web::Data<Box<dyn Greeter>>) -> impl Responder {
                        
                        let buffer = base64::decode(body).unwrap();
                        let s = #req_message::decode(buffer.as_ref()).unwrap();
                    
                        let reply = greeter.into_inner().say_hello(s).await.unwrap();
                    
                        let mut proto_buffer: Vec<u8> = Vec::new();
                        reply.encode(&mut proto_buffer).unwrap();
                        base64::encode(proto_buffer)
                    }
                }
            }
            (true, false) => {
                TokenStream::new()
            }
            (false, true) => {
                TokenStream::new()
            }
            (true, true) => {
                TokenStream::new()
            }
        };

        stream.extend(method);
    }

    stream
}
