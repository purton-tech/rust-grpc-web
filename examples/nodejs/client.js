const {HelloRequest, HelloReply} = require('./helloworld_pb.js');
const {GreeterClient} = require('./helloworld_grpc_web_pb.js');

var request = new HelloRequest();
var echoService = new GreeterClient('http://localhost:8080');

var request = new HelloRequest();
request.setName('Hello World!');

echoService.sayHello(request, {}, function(err, response) {
  // ...
});