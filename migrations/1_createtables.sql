DROP SCHEMA IF EXISTS pfe CASCADE;
CREATE SCHEMA pfe;

CREATE TABLE IF NOT EXISTS pfe.questions (
    id SERIAL PRIMARY KEY,
    category VARCHAR(255) NOT NULL,
    sub_category VARCHAR(255) NOT NULL,
    question TEXT NOT NULL,
    is_used BOOLEAN DEFAULT TRUE
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
    business_activity TEXT,
    nb_workers INTEGER,
    revenue DOUBLE PRECISION,
    labels TEXT,
    dispute BOOLEAN DEFAULT FALSE
);

CREATE TABLE IF NOT EXISTS pfe.template_company (
    company_id INTEGER REFERENCES pfe.companies(company_id),
    template_id INTEGER REFERENCES pfe.templates(template_id),
    PRIMARY KEY (company_id, template_id)
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

CREATE TABLE IF NOT EXISTS pfe.user_answer_esg (
    answer_id INTEGER REFERENCES pfe.answers(answer_id),
    user_id INTEGER REFERENCES pfe.users(user_id),
    form_id INTEGER REFERENCES pfe.forms(form_id),
    now BOOLEAN NOT NULL,
    commitment_pact BOOLEAN NOT NULL,
    comment TEXT,
    now_verif BOOLEAN ,
    commitment_pact_verif BOOLEAN,
    PRIMARY KEY (answer_id, user_id, form_id)
);

