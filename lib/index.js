var Connection = require('./connection').Connection;

var User = require('../native').User;

var lumbergh = new User(9001, "Bill", "Lumbergh", "bill@example.com");
console.log(lumbergh.get("id"));

var c = new Connection();
console.log(c.hello());
console.log(c.test_connection());
console.log(c.test_new_connection());
