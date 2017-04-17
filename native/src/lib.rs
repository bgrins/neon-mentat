
#![allow(unused_variables)]

#[macro_use]
extern crate neon;
extern crate mentat;
extern crate mentat_db;
extern crate rusqlite;

use neon::mem::Handle;
use neon::vm::Lock;
use neon::vm::{Call, JsResult};

use neon::js::{JsString, JsNumber, Object};
use neon::js::class::Class;

use neon::js::error::{JsError, Kind};
use mentat::{new_connection, q_once, conn}; // QueryResults

use std::rc::Rc;
use std::cell::RefCell;


pub struct Connection {
    rusqlite_connection: Rc<RefCell<rusqlite::Connection>>,
    conn: Rc<RefCell<conn::Conn>>,
}

declare_types! {
  pub class JsConnection for Connection {
        init(call) {
            // TODO: Receive path in constructor
            let scope = call.scope;
            // let path: Handle<JsString> = try!(try!(call.arguments.require(scope, 0)).check::<JsString>());

            let c = Rc::new(RefCell::new(new_connection("").expect("Couldn't open conn.")));
            let conn = Rc::new(RefCell::new(conn::Conn::connect(&mut c.borrow_mut()).expect("Couldn't open DB.")));

            Ok(Connection {
                rusqlite_connection: c,
                conn: conn,
            })
        }

        method close(call) {
            let scope = call.scope;

            Ok(try!(JsString::new_or_throw(scope, &"Not implemented"[..])).upcast())
        }

        // TODO: Take in parameters and pass back results
        method transact(call) {
            let scope = call.scope;
            let input: Handle<JsString> = try!(try!(call.arguments.require(scope, 0)).check::<JsString>());

            let mut args1 = call.arguments.this(scope);
            let mut rusqlite_connection = args1.grab(|user| { user.rusqlite_connection.borrow_mut() });
            let mut args2 = call.arguments.this(scope);
            let mut db = args2.grab(|user| { user.conn.borrow_mut() });

            let results = &db.transact(&mut rusqlite_connection,
                                input.value().as_str()).expect("Query failed");

            Ok(try!(JsString::new_or_throw(scope, &results.tx_id.to_string()[..])).upcast())
        }

        // TODO: Take in parameters and pass back results
        method query(call) {
            let scope = call.scope;
            let input: Handle<JsString> = try!(try!(call.arguments.require(scope, 0)).check::<JsString>());

            let mut args1 = call.arguments.this(scope);
            let rusqlite_connection = args1.grab(|user| { user.rusqlite_connection.borrow_mut() });
            let mut args2 = call.arguments.this(scope);
            let db = args2.grab(|user| { user.conn.borrow_mut() });

            let results = &db.q_once(&rusqlite_connection,
                                input.value().as_str(),
                                None,
                                None)
                .expect("Query failed");

            Ok(try!(JsString::new_or_throw(scope, &results.len().to_string()[..])).upcast())
        }
  }
}

pub struct User {
    id: i32,
    first_name: String,
    last_name: String,
    email: String,
}

declare_types! {
  pub class JsUser for User {
        init(call) {
        let scope = call.scope;
        let id = try!(try!(call.arguments.require(scope, 0)).check::<JsNumber>());
        let first_name: Handle<JsString> = try!(try!(call.arguments.require(scope, 1)).check::<JsString>());
        let last_name: Handle<JsString> = try!(try!(call.arguments.require(scope, 2)).check::<JsString>());
        let email: Handle<JsString> = try!(try!(call.arguments.require(scope, 3)).check::<JsString>());

        Ok(User {
            id: id.value() as i32,
            first_name: first_name.value(),
            last_name: last_name.value(),
            email: email.value(),
        })
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

    let user_class = try!(JsUser::class(m.scope));
    let user_constructor = try!(user_class.constructor(m.scope));
    try!(m.exports.set("User", user_constructor));

    let connection_class = try!(JsConnection::class(m.scope));
    let connection_constructor = try!(connection_class.constructor(m.scope));
    try!(m.exports.set("Connection", connection_constructor));

    Ok(())
});
