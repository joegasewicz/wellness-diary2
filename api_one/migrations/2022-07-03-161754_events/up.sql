-- Your SQL goes here
CREATE TABLE users (
    id BIGSERIAL PRIMARY KEY,
    email VARCHAR(300) NOT NULL,
    password VARCHAR(20) NOT NULL,
    name VARCHAR(100),
    CONSTRAINT user_email_unique UNIQUE (email)
);

CREATE TABLE categories (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    CONSTRAINT cat_name_unique UNIQUE (name)
);


CREATE TABLE symptom_types (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    user_id BIGSERIAL REFERENCES users (id),
    CONSTRAINT sym_name_unique UNIQUE (name)
);

CREATE TABLE events (
    id SERIAL PRIMARY KEY,
    details VARCHAR(3000) NOT NULL,
    date timestamp with time zone,
    user_id BIGSERIAL REFERENCES users (id),
    category_id integer REFERENCES categories (id)
);

CREATE TABLE symptoms (
    id SERIAL PRIMARY KEY,
    date timestamp with time zone,
    details VARCHAR(3000),
    user_id BIGSERIAL REFERENCES users (id),
    symptom_types_id integer REFERENCES symptom_types (id)
);


