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