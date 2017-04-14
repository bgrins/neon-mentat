#[macro_use]
extern crate neon;
extern crate mentat;

use neon::vm::{Call, JsResult};
use neon::js::JsString;

fn hello(call: Call) -> JsResult<JsString> {
    let scope = call.scope;
    let s = JsString::new(scope, "hello node!").unwrap();
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
    Ok(())
});
