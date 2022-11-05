-- Your SQL goes here
CREATE TABLE "quote_subject" (
    id             INTEGER      PRIMARY KEY AUTOINCREMENT,
    quote_singular VARCHAR (255) NOT NULL,
    quote_plural   VARCHAR (255) NOT NULL
);

CREATE TABLE "quote_verb" (
    id             INTEGER      PRIMARY KEY AUTOINCREMENT,
    quote_singular VARCHAR (255) NOT NULL,
    quote_plural   VARCHAR (255) NOT NULL
);

CREATE TABLE "quote_description" (
    id             INTEGER      PRIMARY KEY AUTOINCREMENT,
    quote_singular VARCHAR (255) NOT NULL,
    quote_plural   VARCHAR (255) NOT NULL
);
