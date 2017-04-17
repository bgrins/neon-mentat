
console.log("Using the Connection class");
var Connection = require('../native').Connection;
var c = new Connection();
console.log(c.query("[:find ?x ?ident :where [?x :db/ident ?ident]]"));
