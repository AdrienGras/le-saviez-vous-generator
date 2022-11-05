// @generated automatically by Diesel CLI.

diesel::table! {
    quote_description (id) {
        id -> Integer,
        quote_singular -> Text,
        quote_plural -> Text,
    }
}

diesel::table! {
    quote_subject (id) {
        id -> Integer,
        quote_singular -> Text,
        quote_plural -> Text,
    }
}

diesel::table! {
    quote_verb (id) {
        id -> Integer,
        quote_singular -> Text,
        quote_plural -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    quote_description,
    quote_subject,
    quote_verb,
);
