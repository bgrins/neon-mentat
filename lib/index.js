var fixtures = require("../test/fixtures")
var Connection = require("./connection");
var conn = new Connection();
conn.transact(fixtures.schema);
conn.transact(fixtures.data);

var input = `
    [:find ?born
    :where
    [?p :person/born ?born]]`;

console.log(conn.query(input));

var input2 = `
    [:find ?name
    :where
    [?p :person/name ?name]]`;

console.log(conn.query(input2));
