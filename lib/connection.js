let Connection;
try {
    Connection = require("../native").Connection;
} catch (e) {
    // If for some reason the native build didn't happen, return an object
    // with the same API
    Connection = function() {
        console.log("No native module available. Using a stubbed Connection object");
    };
    Connection.prototype = {
        IS_STUBBED: true,
        query: function() {
            return { results: [], resultsLength: 0 };
        },
        transact: function() {
            return 0;
        },
        close: function() { },
    };
}

module.exports = Connection;
