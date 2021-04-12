import { grpc } from "@improbable-eng/grpc-web";
const {SubscribeRequest, CurrenciesRequest} = require('./quotes_pb.js');
const {QuoteServiceClient} = require('./quotes_pb_service.js');

const client = new QuoteServiceClient('http://localhost:8080', {
  transport: grpc.WebsocketTransport()
});

const request = new CurrenciesRequest();

client.getCurrencies(request, {}, (err, response) => {
  console.log(err)
  console.log(response.getIsoCodesList());
});

const subscribeRequest = new SubscribeRequest();

const stream = client.subscribe(request);

stream.on('data', function(response) {
  document.getElementById('price').value = response.getKey()
});
stream.on('status', function(status) {
  console.log(status.code);
  console.log(status.details);
  console.log(status.metadata);
});
stream.on('end', function(end) {
  // stream end signal
});