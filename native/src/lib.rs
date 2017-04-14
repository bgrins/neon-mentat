#[macro_use]
extern crate neon;
extern crate mentat;
extern crate mentat_db;

use neon::vm::{Call, JsResult};
use neon::js::JsString;
use mentat::{
    new_connection,
    q_once,
};


fn hello(call: Call) -> JsResult<JsString> {
    let scope = call.scope;
    let s = JsString::new(scope, "hello node!").unwrap();
    Ok(s)
}

fn test_new_connection(call: Call) -> JsResult<JsString> {
    let mut c = new_connection("").expect("Couldn't open conn.");
    let db = mentat_db::db::ensure_current_version(&mut c).expect("Couldn't open DB.");
    let scope = call.scope;

    let results = q_once(&c, &db.schema,
                         "[:find ?x ?ident :where [?x :db/ident ?ident]]", None, None)
        .expect("Query failed");

    let s = JsString::new(scope, &results.len().to_string()).unwrap();
    Ok(s)
}

fn test_connection(call: Call) -> JsResult<JsString> {
    #[derive(Debug)]
    struct Person {
        id: i32,
        name: String,
        data: Option<Vec<u8>>,
    }

    let conn = mentat::get_connection();
    conn.execute("CREATE TABLE person (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL,
                  data            BLOB
                  )",
                 &[])
        .unwrap();
    let me = Person {
        id: 1,
        name: "Steven".to_string(),
        data: None,
    };
    conn.execute("INSERT INTO person (name, data)
                  VALUES (?1, ?2)",
                 &[&me.name, &me.data])
        .unwrap();

    let mut stmt = conn.prepare("SELECT id, name, data FROM person").unwrap();
    let person_iter = stmt.query_map(&[], |row| {
            Person {
                id: row.get(0),
                name: row.get(1),
                data: row.get(2),
            }
        })
        .unwrap();


    let scope = call.scope;
    let mut s = JsString::new(scope, "hello node!").unwrap();
    for person in person_iter {
        let p = person.unwrap();
        s = JsString::new(scope, &p.name.to_string()).unwrap();
    }

    Ok(s)
}

register_module!(m, {
    try!(m.export("hello", hello));
    try!(m.export("test_connection", test_connection));
    try!(m.export("test_new_connection", test_new_connection));
    Ok(())
});
