use crate::schema::quote_subject::dsl::*;
use crate::models::QuoteSubject;
use diesel::{SqliteConnection, QueryDsl, RunQueryDsl};
use crate::helpers::database;

use diesel::prelude::*;

fn get_connection() -> SqliteConnection {
    database::establish_connection()
}

pub fn count_all() -> i64 {
    let db: &mut SqliteConnection = &mut get_connection();

    quote_subject
    .count()
    .get_result::<i64>(db)
    .unwrap()
}

pub fn find(object_id: i64) -> QuoteSubject {
    let db: &mut SqliteConnection = &mut get_connection();

    let clear_id: i32 = object_id.try_into().unwrap();

    quote_subject
    .filter(id.eq(clear_id))
    .first(db)
    .unwrap()
}