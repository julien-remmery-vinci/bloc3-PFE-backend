DROP SCHEMA IF EXISTS pfe CASCADE;
CREATE SCHEMA pfe;

CREATE TABLE IF NOT EXISTS pfe.questions (
    question_id SERIAL PRIMARY KEY,
    category VARCHAR(255) NOT NULL,
    sub_category VARCHAR(255) NOT NULL,
    question TEXT NOT NULL,
    is_used BOOLEAN DEFAULT TRUE,
    question_type TEXT NOT NULL CHECK (question_type IN ('ODD', 'ESG'))
);

CREATE TABLE IF NOT EXISTS pfe.templates (
    template_id SERIAL PRIMARY KEY,
    value TEXT NOT NULL CHECK (value IN ('ALL', 'OWNED FACILITY', 'WORKERS', 'PRODUITS', 'FACILITY'))
);

CREATE TABLE IF NOT EXISTS pfe.companies (
    company_id SERIAL PRIMARY KEY,
    company_name VARCHAR(255) NOT NULL,
    company_number VARCHAR(50) UNIQUE,
    legal_form VARCHAR(100),
    office_address TEXT,
    website VARCHAR(255),
    nace_code VARCHAR(20),
    nb_workers INTEGER,
    revenue DOUBLE PRECISION,
    dispute BOOLEAN DEFAULT FALSE
);

CREATE TABLE IF NOT EXISTS pfe.template_company (
    company_id INTEGER REFERENCES pfe.companies(company_id),
    template_id INTEGER REFERENCES pfe.templates(template_id),
    PRIMARY KEY (company_id, template_id)
);

CREATE TABLE IF NOT EXISTS pfe.forms (
    form_id SERIAL PRIMARY KEY,
    company_id INTEGER REFERENCES pfe.companies(company_id),
    type VARCHAR(10) CHECK (type IN ('ODD', 'ESG')),
    status VARCHAR(10) CHECK (status IN ('PENDING', 'SUBMITTED', 'VALIDATED'))
);

CREATE TABLE IF NOT EXISTS pfe.template_form (
    form_id INTEGER REFERENCES pfe.forms(form_id),
    template_id INTEGER REFERENCES pfe.templates(template_id),
    PRIMARY KEY (form_id, template_id)
);

CREATE TABLE IF NOT EXISTS pfe.users (
    user_id SERIAL PRIMARY KEY,
    firstname TEXT NOT NULL,
    lastname TEXT NOT NULL,
    login TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL,
    role TEXT NOT NULL CHECK (role IN ('user', 'admin')),
    company_id INTEGER REFERENCES pfe.companies(company_id) NULL
);

CREATE TABLE IF NOT EXISTS pfe.questions_form (
    form_id INTEGER REFERENCES pfe.forms(form_id),
    question_id INTEGER REFERENCES pfe.questions(question_id),
    question_status TEXT NOT NULL CHECK (question_status IN ('PENDING', 'COMPLETE')),
    PRIMARY KEY (form_id, question_id)
);

CREATE TABLE IF NOT EXISTS pfe.answers_esg (
    answer_id SERIAL PRIMARY KEY,
    question_id INTEGER REFERENCES pfe.questions(question_id),
    template TEXT NOT NULL,
    answer TEXT,
    score_now DOUBLE PRECISION NULL,
    score_commitment_pact DOUBLE PRECISION NULL,
    is_forced_engagement BOOLEAN NOT NULL DEFAULT FALSE,
    is_forced_comment BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE IF NOT EXISTS pfe.user_answer_esg (
    answer_id INTEGER REFERENCES pfe.answers_esg(answer_id),
    user_id INTEGER REFERENCES pfe.users(user_id),
    form_id INTEGER REFERENCES pfe.forms(form_id),
    now BOOLEAN NULL,
    commitment_pact BOOLEAN NULL,
    comment TEXT NULL,
    now_verif BOOLEAN NULL,
    commitment_pact_verif BOOLEAN NULL,
    status TEXT CHECK (status IN ('PENDING', 'VALIDATED')),
    PRIMARY KEY (answer_id, user_id, form_id)
);

CREATE TABLE IF NOT EXISTS pfe.onboarding (
    onboarding_id SERIAL PRIMARY KEY,
    firstname TEXT NOT NULL,
    lastname TEXT NOT NULL,
    email TEXT NOT NULL,
    password TEXT NOT NULL,
    position TEXT NOT NULL,
    company_name TEXT NOT NULL,
    company_number TEXT NOT NULL,
    legal_form TEXT NOT NULL,
    office_address TEXT NOT NULL,
    website TEXT NOT NULL,
    nace_code TEXT NOT NULL,
    revenue DOUBLE PRECISION NOT NULL,
    franchise BOOLEAN NOT NULL,
    nb_workers INTEGER NOT NULL,
    dispute BOOLEAN NOT NULL,
    honor_engagement BOOLEAN NOT NULL,
    grant_application BOOLEAN NOT NULL,
    grant_application_partner TEXT NOT NULL,
    something_else TEXT NOT NULL,
    comment TEXT NOT NULL,
    submit_date TEXT NOT NULL DEFAULT NOW(),
    is_owner BOOLEAN NOT NULL,
    offers_services BOOLEAN NOT NULL,
    sells_products BOOLEAN NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('PENDING', 'ACCEPTED', 'REJECTED')) DEFAULT 'PENDING'
);
