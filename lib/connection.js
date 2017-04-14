var addon = require('../native');

function Connection() {
    
}
Connection.prototype.test_new_connection = function() {
    return addon.test_new_connection();
}
Connection.prototype.test_connection = function() {
    return addon.test_connection();
}
Connection.prototype.hello = function() {
    return addon.hello();
}
exports.Connection = Connection;