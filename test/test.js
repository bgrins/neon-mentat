var Connection = require("../lib/connection");
var assert = require('assert');
var fixtures = require("./fixtures");

describe('Connection', function () {
    describe('basics', function () {
        var conn = new Connection();
        it('should transact', function () {
            var input = "[]";
            var response = conn.transact(input);
            assert.equal(response, 0x10000000 + 1);
        });
        it('should query', function () {
            var input = "[:find ?x ?ident :where [?x :db/ident ?ident]]";
            var response = conn.query(input);
            assert.equal(response.results.length, 39);
        });
        it('should close', function () {
            assert.equal(conn.close(), "Not implemented");
        });
    });
    describe('with data', function () {
        var conn = new Connection();
        it('should transact (1)', function () {
            var input = fixtures.schema;
            var response = conn.transact(input);
            assert.equal(response, 0x10000000 + 1);
        });
        it('should transact (2)', function () {
            var input = fixtures.data;
            var response = conn.transact(input);
        });
        it('should query (Scalar - Some)', function() {
            var input = `[:find ?ident . :where [24 :db/ident ?ident]]`;
            var response = conn.query(input);
            assert.equal(response.results[0], ':db.type/keyword');
            assert.equal(response.results.length, 1);
        });
        it('should query (Scalar - None)', function() {
            var input = `[:find ?ident . :where [10000 :db/ident ?ident]]`;
            var response = conn.query(input);
            assert.equal(response.results.length, 0);
            assert.equal(response.results.length, 0);
        });
        it('should query (Coll)', function() {
            var input = `[:find [?e ...] :where [?e :db/ident _]]`;
            var response = conn.query(input);
            assert.equal(response.results.length, 48);
        });
        it('should query (1)', function () {
            var input = "[:find ?x ?ident :where [?x :db/ident ?ident]]";
            var response = conn.query(input);
            assert.equal(response.results.length, 48);
        });
        it('should query (2)', function () {
            var input = `
                [:find ?e
                :where
                [?e :person/name "James Cameron" _]]`;
            var response = conn.query(input);
            assert.equal(response.results.length, 1);
        });

        it('should query (3)', function () {
            var input = `
                [:find ?name
                :where
                [?p :person/name ?name]]`;
            var response = conn.query(input);
            assert.deepEqual(response.results, [['Sophie Marceau' ],[ 'Tina Turner' ],[ 'George Ogilvie' ],[ 'Bruce Spence' ],[ 'Michael Preston' ],[ 'Joanne Samuel' ],[ 'Steve Bisley' ],[ 'George Miller' ],[ 'Carrie Henn' ],[ 'Veronica Cartwright' ],[ 'Sigourney Weaver' ],[ 'Tom Skerritt' ],[ 'Ridley Scott' ],[ 'Joe Pesci' ],[ 'Ruben Blades' ],[ 'Stephen Hopkins' ],[ 'Marc de Jonge' ],[ 'Peter MacDonald' ],[ 'Charles Napier' ],[ 'George P. Cosmatos' ],[ 'Claire Danes' ],[ 'Nick Stahl' ],[ 'Jonathan Mostow' ],[ 'Edward Furlong' ],[ 'Robert Patrick' ],[ 'Alexander Godunov' ],[ 'Alan Rickman' ],[ 'Bruce Willis' ],[ 'Alyssa Milano' ],[ 'Rae Dawn Chong' ],[ 'Mark L. Lester' ],[ 'Ronny Cox' ],[ 'Nancy Allen' ],[ 'Peter Weller' ],[ 'Paul Verhoeven' ],[ 'Gary Busey' ],[ 'Danny Glover' ],[ 'Mel Gibson' ],[ 'Richard Donner' ],[ 'Carl Weathers' ],[ 'Elpidia Carrillo' ],[ 'John McTiernan' ],[ 'Brian Dennehy' ],[ 'Richard Crenna' ],[ 'Sylvester Stallone' ],[ 'Ted Kotcheff' ],[ 'Michael Biehn' ],[ 'Linda Hamilton' ],[ 'Arnold Schwarzenegger' ],[ 'James Cameron' ] ]);
        });

        it('should query (4)', function () {
            var input = `
                [:find ?e
                :where
                [?e :movie/year 1987]
                [?e :movie/title ?title]]
            `;
            var response = conn.query(input);
            assert.equal(response.results.length, 3);
        });


        it('should close', function () {
            assert.equal(conn.close(), "Not implemented");
        });

    });
});

