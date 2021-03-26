const {HelloRequest, HelloReply} = require('./helloworld_pb.js');
const {GreeterClient} = require('./helloworld_grpc_web_pb.js');

var client = new GreeterClient('http://localhost:8080');

var request = new HelloRequest();
request.setName('World');

console.log(request.serializeBinary())

client.sayHello(request, {}, (err, response) => {
  console.log(err)
  console.log(response.getMessage());
});