/*
 * Copyright (c) 2017 Boucher, Antoni <bouanto@zoho.com>
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of
 * this software and associated documentation files (the "Software"), to deal in
 * the Software without restriction, including without limitation the rights to
 * use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 * the Software, and to permit persons to whom the Software is furnished to do so,
 * subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
 * FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
 * COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
 * IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#![feature(proc_macro)]

extern crate tql;
#[macro_use]
extern crate tql_macros;

#[cfg(feature = "postgres")]
use postgres::error::UNDEFINED_TABLE;
use tql::{ForeignKey, PrimaryKey};
use tql_macros::sql;

use connection::get_connection;

#[macro_use]
mod connection;
mod teardown;

backend_extern_crate!();

use teardown::TearDown;

#[derive(SqlTable)]
struct TableConnectionExpr {
    primary_key: PrimaryKey,
    field1: String,
    field2: i32,
    related_field: ForeignKey<RelatedTableConnectionExpr>,
    optional_field: Option<i32>,
    boolean: Option<bool>,
    //character: Option<char>, // TODO: does not work.
    float64: Option<f64>,
    //int8: Option<i8>, // TODO: does not work.
    int16: Option<i16>,
    int64: Option<i64>,
}

#[derive(SqlTable)]
struct RelatedTableConnectionExpr {
    primary_key: PrimaryKey,
    field1: i32,
}

#[test]
fn test_insert() {
    let cx = get_connection();

    let _teardown = TearDown::new(|| {
        let _ = sql!(cx, TableConnectionExpr.drop());
        let _ = sql!(cx, RelatedTableConnectionExpr.drop());
    });

    let _ = sql!(cx, RelatedTableConnectionExpr.create());
    let _ = sql!(cx, TableConnectionExpr.drop());

    let related_id = sql!(cx, RelatedTableConnectionExpr.insert(field1 = 42)).unwrap();
    let related_field = sql!(cx, RelatedTableConnectionExpr.get(related_id)).unwrap();

    let result = sql!(cx, TableConnectionExpr.insert(field1 = "value1", field2 = 55, related_field = related_field));
    match result {
        Err(db_error) => {
            #[cfg(feature = "postgres")]
            assert_eq!(Some(&UNDEFINED_TABLE), db_error.code());
            #[cfg(feature = "sqlite")]
            assert_eq!(db_error.to_string(), "no such table: TableConnectionExpr");
        },
        Ok(_) => assert!(false),
    }

    let _ = sql!(cx, TableConnectionExpr.create());

    let id = sql!(cx, TableConnectionExpr.insert(field1 = "value1", field2 = 55, related_field = related_field)).unwrap();
    assert_eq!(1, id);

    let table = sql!(cx, TableConnectionExpr.get(id)).unwrap();
    assert_eq!("value1", table.field1);
    assert_eq!(55, table.field2);
    assert!(table.related_field.is_none());
    assert!(table.optional_field.is_none());

    let table = sql!(cx, TableConnectionExpr.get(id).join(related_field)).unwrap();
    assert_eq!("value1", table.field1);
    assert_eq!(55, table.field2);
    let related_table = table.related_field.unwrap();
    assert_eq!(related_id, related_table.primary_key);
    assert_eq!(42, related_table.field1);
    assert!(table.optional_field.is_none());

    let new_field2 = 42;
    let id = sql!(cx, TableConnectionExpr.insert(field1 = "value2", field2 = new_field2, related_field = related_field)).unwrap();
    assert_eq!(2, id);

    let table = sql!(cx, TableConnectionExpr.get(id)).unwrap();
    assert_eq!("value2", table.field1);
    assert_eq!(42, table.field2);
    assert!(table.related_field.is_none());
    assert!(table.optional_field.is_none());

    let new_field1 = "value3".to_string();
    let new_field2 = 24;
    let id = sql!(cx, TableConnectionExpr.insert(
        field1 = new_field1,
        field2 = new_field2,
        related_field = related_field,
        optional_field = Some(12),
    )).unwrap();
    assert_eq!(3, id);

    let table = sql!(cx, TableConnectionExpr.get(id)).unwrap();
    assert_eq!("value3", table.field1);
    assert_eq!(24, table.field2);
    assert!(table.related_field.is_none());
    assert_eq!(Some(12), table.optional_field);

    let connection = &cx;
    let boolean_value = true;
    //let character = 'a';
    let float64 = 3.14f64;
    //let int8 = 42i8;
    let int16 = 42i16;
    let int64 = 42i64;
    let id = sql!(TableConnectionExpr.insert(
        field1 = new_field1,
        field2 = new_field2,
        related_field = related_field,
        optional_field = Some(12),
        boolean = Some(boolean_value),
        /*character = character,*/
        float64 = Some(float64),
        /*int8 = int8,*/
        int16 = Some(int16),
        int64 = Some(int64)
    )).unwrap();
    assert_eq!(4, id);
}
