
#![allow(unused_variables)]
#![allow(unused_imports)]

#[macro_use]
extern crate neon;
extern crate mentat;
extern crate mentat_db;
extern crate mentat_core;
extern crate rusqlite;

use mentat_core::{TypedValue, ValueType};
use neon::mem::Handle;
use neon::vm::Lock;
use neon::vm::{Call, JsResult};

use neon::js::{JsString, JsNumber, Object, JsArray, JsObject, JsInteger};
use neon::js::class::Class;

use neon::js::error::{JsError, Kind};
use mentat::{new_connection, conn, QueryResults};

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
            let mut rusqlite_connection = args1.grab(|connection| { connection.rusqlite_connection.borrow_mut() });
            let mut args2 = call.arguments.this(scope);
            let mut db = args2.grab(|connection| { connection.conn.borrow_mut() });

            let results = &db.transact(&mut rusqlite_connection,
                                input.value().as_str()).expect("Query failed");

            Ok(try!(JsString::new_or_throw(scope, &results.tx_id.to_string()[..])).upcast())
        }

        // TODO: Take in parameters and pass back results
        method query(call) {
            let scope = call.scope;
            let output = JsObject::new(scope);

            let input: Handle<JsString> = try!(try!(call.arguments.require(scope, 0)).check::<JsString>());

            let mut args1 = call.arguments.this(scope);
            let rusqlite_connection = args1.grab(|connection| { connection.rusqlite_connection.borrow_mut() });
            let mut args2 = call.arguments.this(scope);
            let db = args2.grab(|connection| { connection.conn.borrow_mut() });

            let results = &db.q_once(&rusqlite_connection,
                                input.value().as_str(),
                                None,
                                None)
                .expect("Query failed");

            if let &QueryResults::Scalar(Some(TypedValue::Keyword(ref rc))) = results {
                println!("Matched Scalar: {:?}", rc);
            }

            if let &QueryResults::Rel(ref rel) = results {
                println!("Matched Rel: {}", rel.len());
                for r in rel {
                    println!("Rel: {:?}", r);
                }
            }

            if let &QueryResults::Tuple(Some(ref tuple)) = results {
                println!("Matched Tuple: {:?}", tuple);
            }

            if let &QueryResults::Coll(ref coll) = results {
                // println!("Matched Coll: {:?}", coll);
                let iter = coll.iter();
                let array: Handle<JsArray> = JsArray::new(scope, iter.len() as u32);
                let mut i = 0;
                for item in iter {
                    let neon_value = match item {
                        &TypedValue::Ref(id) => JsString::new_or_throw(scope, id.to_string().as_str()),
                        _ => JsString::new_or_throw(scope, &"Not implemented"[..]),
                        // &TypedValue::String(s) => JsString::new_or_throw(scope, s)
                        // &TypedValue::Boolean(b) => JsString::new_or_throw(scope, b)
                        // &TypedValue::String(s) => JsString::new_or_throw(scope, s)
                    };

                    try!(array.set(i, neon_value.unwrap()));
                    i = i+1;
                }

                try!(output.set("results", array));
            }

            try!(output.set("resultsLength", JsInteger::new(scope, results.len() as i32)));
            Ok(output.upcast())
        }
  }
}

register_module!(m, {
    let connection_class = try!(JsConnection::class(m.scope));
    let connection_constructor = try!(connection_class.constructor(m.scope));
    try!(m.exports.set("Connection", connection_constructor));

    Ok(())
});
