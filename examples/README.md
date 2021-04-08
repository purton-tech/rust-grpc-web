## Helloworld

### Rust Actix-Web Server

```bash
$ cd helloworld
$ cargo run --bin helloworld-server
```

Test the server

`curl -X POST --data "AAAAAAcKBVdvcmxk" localhost:8080/helloworld.Greeter/SayHello`

### Rust CLI Client

```bash
$ cd helloworld
$ cargo run --bin helloworld-client
```

### Javascript Client (In browser)

You'll need to start the Rust Actix-Web server then build and run the client.

```bash
$ cd js-client
$ sudo apt update
$ sudo apt install -y protobuf-compiler
$ npm i protoc-gen-grpc-web
$ npm install
$ npm run protoc
$ npm run start
```

Open your browser at `localhost:9000`

### Seed.rs Webassembly Client

```bash
$ cd seed-client
$ cargo install cargo-make
```

1. Watch compilation in the terminal tab where you run `cargo make watch`.
1. You can watch dev-server responses in the tab where you run `cargo make serve`.

Open your browser at `localhost:8000`, click the button and in dev tools look at the console.

### Yew Webassembly Client

```bash
$ cargo install --locked trunk
$ trunk serve --port=8081
```

Open your browser at `localhost:8081`

### tonic-key-value-store

This examples contains a simple key/value store with a gRPC API and client built with tonic.

### Running the example

Running a server:

```
RUST_LOG=tonic_key_value_store=trace,tower_http=trace \
    cargo run --bin tonic-key-value-store -- -p 3000 server
```

Setting values:

```
echo "Hello, World" | cargo run --bin tonic-key-value-store -- -p 3000 set -k foo
```

Getting values:

```
cargo run --bin tonic-key-value-store -- -p 3000 get -k foo
```

Create a stream of new keys:

```
cargo run --bin tonic-key-value-store -- -p 3000 subscribe
```
