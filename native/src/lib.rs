
#![allow(unused_variables)]
#![allow(unused_imports)]

extern crate chrono;
extern crate mentat;

#[macro_use]
extern crate neon;
extern crate rusqlite;

use chrono::{
    DateTime,
    UTC,
};

use neon::mem::Handle;
use neon::vm::Lock;
use neon::vm::{Call, JsResult};

use neon::js::{JsString, JsNumber, Object, JsArray, JsObject, JsInteger, JsBoolean, JsValue};
use neon::js::class::Class;

use neon::js::error::{JsError, Kind};

use mentat::{
    Conn,
    QueryResults,
    TypedValue,
    ValueType,
    conn,
    new_connection,
};

use std::rc::Rc;
use std::cell::RefCell;

pub struct Connection {
    rusqlite_connection: Rc<RefCell<rusqlite::Connection>>,
    conn: Rc<RefCell<Conn>>,
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
                                None)
                .expect("Query failed");

            use QueryResults::*;
            match results {
                &Scalar(None) => {
                    // Query didn't return a result
                    let array: Handle<JsArray> = JsArray::new(scope, 0);
                    try!(output.set("results", array));
                },
                &Scalar(Some(ref v)) => {
                    let array: Handle<JsArray> = JsArray::new(scope, 1);
                    let processed_value = process_typed_value(scope, v);

                    // TODO: Can we match all possible types with a single pattern
                    match processed_value {
                        ReturnedHandle::JsString(handle) => {
                            try!(array.set(0, handle));
                        }
                        ReturnedHandle::JsBoolean(handle) => {
                            try!(array.set(0, handle));
                        }
                        ReturnedHandle::JsNumber(handle) => {
                            try!(array.set(0, handle));
                        }
                    }
                    try!(output.set("results", array));
                },
                &Tuple(None) => {
                    // println!("TODO: Matched Tuple None");
                },
                &Tuple(Some(ref tuple)) => {
                    // println!("TODO: Matched Tuple: {:?}", tuple);
                },
                &Coll(ref coll) => {
                    // println!("Matched Coll: {:?}", coll);
                    let iter = coll.iter();
                    let array: Handle<JsArray> = JsArray::new(scope, iter.len() as u32);
                    for (i, item) in iter.enumerate() {
                        let processed_value = process_typed_value(scope, item);

                        // TODO: Can we match all possible types with a single pattern
                        match processed_value {
                            ReturnedHandle::JsString(handle) => {
                                try!(array.set(i as u32, handle));
                            }
                            ReturnedHandle::JsBoolean(handle) => {
                                try!(array.set(i as u32, handle));
                            }
                            ReturnedHandle::JsNumber(handle) => {
                                try!(array.set(i as u32, handle));
                            }
                        }
                    }

                    try!(output.set("results", array));
                }
                &Rel(ref rel) => {
                    let iter = rel.iter();
                    let array: Handle<JsArray> = JsArray::new(scope, iter.len() as u32);
                    for (i, r) in iter.enumerate() {
                        let r_iter = r.iter();
                        let r_array: Handle<JsArray> = JsArray::new(scope, r_iter.len() as u32);
                        for (j, item) in r_iter.enumerate() {
                            let processed_value = process_typed_value(scope, item);
                            match processed_value {
                                ReturnedHandle::JsString(handle) => {
                                    try!(r_array.set(j as u32, handle));
                                }
                                ReturnedHandle::JsBoolean(handle) => {
                                    try!(r_array.set(j as u32, handle));
                                }
                                ReturnedHandle::JsNumber(handle) => {
                                    try!(r_array.set(j as u32, handle));
                                }
                            }
                        }
                        try!(array.set(i as u32, r_array));
                    }
                    try!(output.set("results", array));
                }
            }

            // TODO: Is this useful?  Length can be gathered via `foo.results.length`, so removing
            // this for now..
            // try!(output.set("resultsLength", JsInteger::new(scope, results.len() as i32)));

            Ok(output.upcast())
        }
  }
}

// TODO: There is probably a more graceful way to handle multiple returned types...
enum ReturnedHandle<'a> {
    JsString(Handle<'a, JsString>),
    JsBoolean(Handle<'a, JsBoolean>),
    JsNumber(Handle<'a, JsNumber>),
}

// ... maybe something like JsResult<'a, JsValue> can be used
// fn process_typed_value<'a, 'b>(scope: &mut neon::scope::RootScope<'a>, item: &'b TypedValue) -> JsResult<'a, JsValue> {
//     let neon_value = match item {
//         &TypedValue::Ref(id) => JsValue(scope, id.to_string().as_str()).unwrap(),
//         &TypedValue::Boolean(b) => JsValue(scope, b),
//     };

//     return neon_value;
// }

fn js_date<'a, 'b>(scope: &mut neon::scope::RootScope<'a>, item: &'b DateTime<UTC>) -> ReturnedHandle<'a> {
    unimplemented!()
}

fn process_typed_value<'a, 'b>(scope: &mut neon::scope::RootScope<'a>, item: &'b TypedValue) -> ReturnedHandle<'a> {
    let neon_value = match item {
        &TypedValue::Ref(id) => ReturnedHandle::JsNumber(JsNumber::new(scope, id as f64)),
        &TypedValue::Boolean(b) => ReturnedHandle::JsBoolean(JsBoolean::new(scope, b)),
        &TypedValue::Long(l) => ReturnedHandle::JsNumber(JsNumber::new(scope, l as f64)),
        &TypedValue::Instant(t) => js_date(scope, &t),
        &TypedValue::Uuid(u) => ReturnedHandle::JsString(JsString::new_or_throw(scope, u.to_string().as_str()).unwrap()),
        &TypedValue::String(ref s) => ReturnedHandle::JsString(JsString::new_or_throw(scope, s.as_str()).unwrap()),
        &TypedValue::Keyword(ref k) => ReturnedHandle::JsString(JsString::new_or_throw(scope, k.to_string().as_str()).unwrap()),
        &TypedValue::Double(d) => ReturnedHandle::JsNumber(JsNumber::new(scope, *d.as_ref())), // TODO: Check if this works
    };

    return neon_value;
}

register_module!(m, {
    let connection_class = try!(JsConnection::class(m.scope));
    let connection_constructor = try!(connection_class.constructor(m.scope));
    try!(m.exports.set("Connection", connection_constructor));

    Ok(())
});
