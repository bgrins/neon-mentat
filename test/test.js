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
