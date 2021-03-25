## Helloworld

### Client

```bash
$ cargo run --bin helloworld-client
```

### Server

```bash
$ cargo run --bin helloworld-server
```

### Test the server

`curl -X POST --data "AAAAAAcKBVdvcmxk" localhost:8080/helloworld.Greeter/SayHello`

Browser geenrates 00 00 00 00 07 0a 05 57 6f 72 6c 64
But we expect                    0a 05 57 6f 72 6c 64

What are the leading 5 bytes?