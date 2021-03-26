# Rust gRPC Web

A Rust implementation of [gRPC][] for browser clients. Useful for webassembly based framewoks such as Yew and Seed.rs.

## Generating Client Code

At the root of your crate, create a build.rs file and add the following code:

```rust
fn main() {
    rust_grpc_web::configure()
        .build_client(true)
        .compile(&["proto/helloworld.proto"], &["proto"])
        .unwrap();
}
```

## Generating Client Code [Experimental]


