use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use crate::schema::*;

#[derive(Identifiable, Queryable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = quote_subject)]
pub struct QuoteSubject {
    pub id: i32,
    pub quote_singular: String,
    pub quote_plural: String
}

#[derive(Queryable)]
pub struct QuoteVerb {
    pub id: i32,
    pub quote_singular: String,
    pub quote_plural: String
}

#[derive(Queryable)]
pub struct QuoteDescription {
    pub id: i32,
    pub quote_singular: String,
    pub quote_plural: String
}