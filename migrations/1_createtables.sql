DROP SCHEMA IF EXISTS pfe CASCADE;
CREATE SCHEMA pfe;

CREATE TABLE IF NOT EXISTS pfe.questions (
    id SERIAL PRIMARY KEY,
    category VARCHAR(255) NOT NULL,
    sub_category VARCHAR(255) NOT NULL,
    question TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS pfe.companies (
    company_id SERIAL PRIMARY KEY,
    company_name VARCHAR(255) NOT NULL,
    company_number VARCHAR(50) UNIQUE,
    legal_form VARCHAR(100),
    office_address TEXT,
    website VARCHAR(255),
    nace_code VARCHAR(20),
    business_activity TEXT,
    nb_workers INTEGER,
    revenue NUMERIC(15, 2),
    labels TEXT,
    dispute BOOLEAN DEFAULT FALSE
);

CREATE TABLE IF NOT EXISTS pfe.users (
    id SERIAL PRIMARY KEY,
    firstname TEXT NOT NULL,
    lastname TEXT NOT NULL,
    login TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL,
    role TEXT NOT NULL CHECK (role IN ('user', 'admin')),
    company_id INTEGER REFERENCES pfe.companies(company_id) NULL
);

CREATE TABLE IF NOT EXISTS pfe.forms (
    form_id SERIAL PRIMARY KEY,
    company INTEGER REFERENCES pfe.companies(company_id),
    type VARCHAR(10) CHECK (type IN ('ODD', 'ESG')),
    nb_questions INTEGER,
    template VARCHAR(15)
);

CREATE TABLE IF NOT EXISTS pfe.questions_form (
    form_id INTEGER REFERENCES pfe.forms(form_id),
    question_id INTEGER REFERENCES pfe.questions(id),
    question_status VARCHAR(255) NOT NULL,
    PRIMARY KEY (form_id, question_id)
);