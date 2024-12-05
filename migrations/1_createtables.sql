CREATE SCHEMA IF NOT EXISTS pfe;

CREATE TABLE IF NOT EXISTS pfe.users (
    id SERIAL PRIMARY KEY,
    login TEXT NOT NULL,
    password TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS pfe.questions (
    id SERIAL PRIMARY KEY,
    question_status VARCHAR(255) NOT NULL,
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

CREATE TABLE IF NOT EXISTS pfe.forms (
    form_id SERIAL PRIMARY KEY,
    company INTEGER REFERENCES pfe.companies(company_id),
    type VARCHAR(10) CHECK (type IN ('ODD', 'ESG')),
    nb_questions INTEGER,
    template TEXT
);

CREATE TABLE IF NOT EXISTS pfe.questions_form (
    form_id INTEGER REFERENCES pfe.forms(form_id),
    question_id INTEGER REFERENCES pfe.questions(id),
    PRIMARY KEY (form_id, question_id)
);

CREATE TABLE IF NOT EXISTS pfe.answers (
    answer_id SERIAL PRIMARY KEY,
    answer TEXT NOT NULL,
    template TEXT NOT NULL,
    question_id INTEGER REFERENCES pfe.questions(id),
    score DOUBLE PRECISION NOT NULL,
    engagement_score DOUBLE PRECISION,
    is_forced_engagement BOOLEAN NOT NULL,
    comment TEXT
);

