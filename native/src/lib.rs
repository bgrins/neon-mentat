#[macro_use]
extern crate neon;
extern crate mentat;
extern crate mentat_db;
extern crate rusqlite;

use neon::mem::Handle;
use neon::vm::Lock;
use neon::vm::{Call, JsResult};

use neon::js::{JsString,JsNumber, Object};
use neon::js::class::{Class};

use neon::js::error::{JsError, Kind};
use mentat::{new_connection, q_once};

pub struct User {
  id: i32,
  first_name: String,
  last_name: String,
  email: String,
  connection: rusqlite::Connection,
  db: mentat_db::DB,
}

declare_types! {
  pub class JsUser for User {
        init(call) {
        let scope = call.scope;
        let id = try!(try!(call.arguments.require(scope, 0)).check::<JsNumber>());
        let first_name: Handle<JsString> = try!(try!(call.arguments.require(scope, 1)).check::<JsString>());
        let last_name: Handle<JsString> = try!(try!(call.arguments.require(scope, 2)).check::<JsString>());
        let email: Handle<JsString> = try!(try!(call.arguments.require(scope, 3)).check::<JsString>());

        let mut c = new_connection("").expect("Couldn't open conn.");
        let db = mentat_db::db::ensure_current_version(&mut c).expect("Couldn't open DB.");
        Ok(User {
            id: id.value() as i32,
            first_name: first_name.value(),
            last_name: last_name.value(),
            email: email.value(),
            connection: c,
            db: db,
        })
        }

        method check_connection(call) {
            let scope = call.scope;

            // XXX: How do we get ahold of the connection without a clone?
            let connection = call.arguments.this(scope).grab(|user| { user.connection });
            let db = call.arguments.this(scope).grab(|user| { user.db });

            let results = q_once(&connection,
                                &db.schema,
                                "[:find ?x ?ident :where [?x :db/ident ?ident]]",
                                None,
                                None)
                .expect("Query failed");

            // let s = JsString::new(scope, &results.len().to_string()).unwrap();
            // Ok(try!(s))
            Ok(try!(JsString::new_or_throw(scope, &results.len().to_string()[..])).upcast())
        }

        method get(call) {
        let scope = call.scope;

        let attr: String = try!(try!(call.arguments.require(scope, 0)).check::<JsString>()).value();

        match &attr[..] {
            "id" => {
            let id = call.arguments.this(scope).grab(|user| { user.id.clone() });
            Ok(JsNumber::new(scope, id as f64).upcast())
            },
            "first_name" => {
            let first_name = call.arguments.this(scope).grab(|user| { user.first_name.clone() });
            Ok(try!(JsString::new_or_throw(scope, &first_name[..])).upcast())
            },
            "last_name" => {
            let last_name = call.arguments.this(scope).grab(|user| { user.last_name.clone() });
            Ok(try!(JsString::new_or_throw(scope, &last_name[..])).upcast())
            },
            "email" => {
            let email = call.arguments.this(scope).grab(|user| { user.email.clone() });
            Ok(try!(JsString::new_or_throw(scope, &email[..])).upcast())
            },
            _ => JsError::throw(Kind::TypeError, "property does not exist")
        }
        }

        method panic(_) {
        panic!("User.prototype.panic")
        }
    }
}

fn hello(call: Call) -> JsResult<JsString> {
    let scope = call.scope;
    let s = JsString::new(scope, "hello node!").unwrap();
    Ok(s)
}

fn test_new_connection(call: Call) -> JsResult<JsString> {
    let mut c = new_connection("").expect("Couldn't open conn.");
    let db = mentat_db::db::ensure_current_version(&mut c).expect("Couldn't open DB.");
    let scope = call.scope;

    let results = q_once(&c,
                         &db.schema,
                         "[:find ?x ?ident :where [?x :db/ident ?ident]]",
                         None,
                         None)
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

    let class = try!(JsUser::class(m.scope));       // get the class
    let constructor = try!(class.constructor(m.scope)); // get the constructor
    try!(m.exports.set("User", constructor));

    Ok(())
});
