
console.log("First, testing the User class");
var User = require('../native').User;
var lumbergh = new User(9001, "Bill", "Lumbergh", "bill@example.com");
console.log(lumbergh.get("id"));
console.log(lumbergh.get("first_name"));
console.log(lumbergh.get("last_name"));
console.log(lumbergh.get("email"));

console.log("Now, testing the Connection class");
var Connection = require('../native').Connection;
var c = new Connection();
console.log(c.query("[:find ?x ?ident :where [?x :db/ident ?ident]]"));
