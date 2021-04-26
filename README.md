# Rust gRPC Web

A Rust implementation of [gRPC][] for browser clients. Useful for webassembly based framewoks such as Yew and Seed.rs.

Creates client stubs that can make gRPC calls. Needs to be used with the gRCP-Web proxy from improbable https://github.com/improbable-eng/grpc-web

## Generating Client Code

```toml
[dependencies]
# for example when using Yew
#yew = "0.17"
#yew-router = "0.14.0"
#yewtil = { version = "0.3.2", features = ["future"] }

# For rust-grpc-web
prost = "0"
reqwest = { version = "0.11" }
web-sys = "0"

[build-dependencies]
rust-grpc-web = { git = "https://github.com/elliptic-email/rust-grpc-web" }
```

At the root of your crate, create a build.rs file and add the following code:

```rust
fn main() {
    rust_grpc_web::configure()
        .compile(&["proto/helloworld.proto"], &["proto"])
        .unwrap();
}
```

## Server side streaming

Still in a proof of concept stage at the moment. See the seed-wasm-client example in the examples folder.


