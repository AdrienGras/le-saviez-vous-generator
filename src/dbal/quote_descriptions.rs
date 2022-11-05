use crate::schema::quote_description::dsl::*;
use crate::models::QuoteDescription;
use diesel::{SqliteConnection, QueryDsl, RunQueryDsl};
use crate::helpers::database;

use diesel::prelude::*;

fn get_connection() -> SqliteConnection {
    database::establish_connection()
}

pub fn count_all() -> i64 {
    let mut db: SqliteConnection = get_connection();

    quote_description
    .count()
    .get_result::<i64>(&mut db)
    .unwrap()
}

pub fn find(object_id: i64) -> QuoteDescription {
    let db: &mut SqliteConnection = &mut get_connection();

    let clear_id: i32 = object_id.try_into().unwrap();

    quote_description
    .filter(id.eq(clear_id))
    .first(db)
    .unwrap()
}