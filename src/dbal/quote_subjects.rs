use crate::schema::quote_subject::dsl::*;
use crate::models::QuoteSubject;
use diesel::{SqliteConnection, QueryDsl, RunQueryDsl};
use crate::helpers::database;

use diesel::prelude::*;

fn get_connection() -> SqliteConnection {
    database::establish_connection()
}

pub fn count_all() -> i64 {
    let mut db: SqliteConnection = get_connection();

    quote_subject
    .count()
    .get_result::<i64>(&mut db)
    .unwrap()
}

pub fn find(object_id: i64) -> QuoteSubject {
    let db: &mut SqliteConnection = &mut get_connection();

    quote_subject
    .filter(id.eq(object_id.into()))
    .first(db)
    .unwrap()
}