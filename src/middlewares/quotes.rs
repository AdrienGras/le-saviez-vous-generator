use base64::{encode, decode};
use rand::Rng;
use crate::dto::quote_dto::QuoteDTO;
use crate::dbal::*;
use crate::models::{QuoteSubject, QuoteVerb, QuoteDescription};
use std::str;

pub fn generate() -> QuoteDTO {
    let mut rng = rand::thread_rng();

    let rnd_subject_id = rng.gen_range(0..quote_subjects::count_all())+1;
    let rnd_verb_id = rng.gen_range(0..quote_verbs::count_all())+1;
    let rnd_description_id = rng.gen_range(0..quote_descriptions::count_all())+1;

    let is_plural = rng.gen_range(1..=2) > 1;

    build_from_ids(
        rnd_subject_id.try_into().unwrap(), 
        rnd_verb_id.try_into().unwrap(), 
        rnd_description_id.try_into().unwrap(), 
        is_plural
    )
}

pub fn generate_from_hash(hash: String) -> QuoteDTO {
    let (subject_id, verb_id, description_id, is_plural) = decipher_hash(hash);

    build_from_ids(subject_id, verb_id, description_id, is_plural)
}

fn build_from_ids(subject_id: i32, verb_id: i32, description_id: i32, is_plural: bool) -> QuoteDTO {
    let subject: String;
    let verb: String;
    let description: String;

    let quote_subject_obj = &quote_subjects::find(subject_id.into());
    let quote_verb_obj = &quote_verbs::find(verb_id.into());
    let quote_description_obj = &quote_descriptions::find(description_id.into());

    if is_plural {
        subject = quote_subject_obj.quote_plural.to_owned();
        verb = quote_verb_obj.quote_plural.to_owned();
        description = quote_description_obj.quote_plural.to_owned();
    } else {
        subject = quote_subject_obj.quote_singular.to_owned();
        verb = quote_verb_obj.quote_singular.to_owned();
        description = quote_description_obj.quote_singular.to_owned();
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

fn generate_hash(subject: &QuoteSubject, verb: &QuoteVerb, description: &QuoteDescription, is_plural: bool) -> String {
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