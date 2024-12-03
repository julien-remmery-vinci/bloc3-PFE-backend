CREATE SCHEMA IF NOT EXISTS pfe;

CREATE TABLE IF NOT EXISTS pfe.hello (
    id SERIAL PRIMARY KEY,
    text TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS pfe.users (
    id SERIAL PRIMARY KEY,
    email TEXT NOT NULL,
    password TEXT NOT NULL
);
