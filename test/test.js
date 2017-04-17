var Connection = require("../native").Connection;
var User = require("../native").User;
var assert = require('assert');

describe('Connection', function() {
  describe('#test_new_connection()', function() {
    var conn = new Connection();
    it('should transact', function() {
        assert.equal(conn.transact("[]"), 0x10000000 + 1);
    });
    it('should query', function() {
        assert.equal(conn.query("[:find ?x ?ident :where [?x :db/ident ?ident]]"), 37);
    });
    it('should close', function() {
        assert.equal(conn.close(), "Not implemented");
    });
  });
});

describe('User (js class implemented in rust', function() {
  describe('basics', function() {
    var user = new User(9001, "Bill", "Lumbergh", "bill@example.com");
    it('should work', function() {
        assert.equal(user.get("id"), 9001);
        assert.equal(user.get("first_name"), "Bill");
        assert.equal(user.get("last_name"), "Lumbergh");
        assert.equal(user.get("email"), "bill@example.com");
    });
  });
});

