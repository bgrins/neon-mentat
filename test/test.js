var Connection = require("../native").Connection;
var User = require("../native").User;
var assert = require('assert');

describe('Connection', function() {
  describe('#test_new_connection()', function() {
    var conn = new Connection();
    it('should work', function() {
        assert.equal(conn.query(), 37);
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

