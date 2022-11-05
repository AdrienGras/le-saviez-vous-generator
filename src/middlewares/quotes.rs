use base64::{encode, decode};
use rand::Rng;
use crate::dto::quote_dto::QuoteDTO;
use crate::dbal::*;
use crate::models::{QuoteSubject, QuoteVerb, QuoteDescription};
use std::str;

pub fn generate() -> QuoteDTO {
    let subject: String = String::new();
    let verb: String = String::new();
    let description: String = String::new();

    let mut rng = rand::thread_rng();

    let count_subjects: i64 = quote_subjects::count_all();
    let count_verbs: i64 = quote_verbs::count_all();
    let count_descriptions: i64 = quote_descriptions::count_all();

    let rnd_subject_id = rng.gen_range(1..count_subjects);
    let rnd_verb_id = rng.gen_range(1..count_verbs);
    let rnd_description_id = rng.gen_range(1..count_verbs);

    let quote_subject_obj = quote_subjects::find(rnd_subject_id);
    let quote_verb_obj = quote_verbs::find(rnd_verb_id);
    let quote_description_obj = quote_descriptions::find(rnd_description_id);

    let is_plural = rng.gen_range(1..2) > 2;

    // generate subject
    if is_plural {
        subject = quote_subject_obj.quote_plural;
        verb = quote_verb_obj.quote_plural;
        description = quote_description_obj.quote_plural;
    } else {
        subject = quote_subject_obj.quote_singular;
        verb = quote_verb_obj.quote_singular;
        description = quote_description_obj.quote_singular;
    }

    let complete_quote: String = format!("{} {} {}", subject, verb, description);

    let hash: String = generate_hash(quote_subject_obj, quote_verb_obj, quote_description_obj, is_plural);

    QuoteDTO { 
        subject, 
        verb, 
        description, 
        complete_quote, 
        hash
    }
}

fn generate_hash(subject: QuoteSubject, verb: QuoteVerb, description: QuoteDescription, is_plural: bool) -> String {
    let clear_hash: String = format!("{}:{}:{}:{}", subject.id, verb.id, description.id, is_plural);

    encode(clear_hash)
}

fn decipher_hash(hash: String) -> (i32, i32, i32, bool) {
    let clear_hash: String = String::from_utf8_lossy(&decode(hash).unwrap()[..]).into_owned();

    let split_vec: Vec<&str> = clear_hash.split(":").collect();

    let subject_id: i32 = split_vec.get(0).unwrap().parse().unwrap();
    let verb_id: i32 = split_vec.get(1).unwrap().parse().unwrap();
    let description_id: i32 = split_vec.get(2).unwrap().parse().unwrap();
    let is_plural: bool = split_vec.get(3).unwrap().parse().unwrap();

    (subject_id, verb_id, description_id, is_plural)
}