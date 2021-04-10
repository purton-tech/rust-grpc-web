const {CurrenciesRequest} = require('./quotes_pb.js');
const {QuoteServiceClient} = require('./quotes_pb_service.js');

var client = new QuoteServiceClient('http://localhost:8080');

var request = new CurrenciesRequest();

console.log(request.serializeBinary())

client.getCurrencies(request, {}, (err, response) => {
  console.log(err)
  console.log(response.getMessage());
});