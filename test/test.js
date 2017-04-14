var Connection = require("../lib/connection").Connection;
var assert = require('assert');

describe('Connection', function() {
  describe('#test_new_connection()', function() {
    var conn = new Connection();
    it('should work', function() {
        assert.equal(conn.test_new_connection(), 37);
    });
  });
});

