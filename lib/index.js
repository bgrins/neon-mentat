var Connection = require('./connection').Connection;

var c = new Connection();
console.log(c.hello());
console.log(c.test_connection());
console.log(c.test_new_connection());
