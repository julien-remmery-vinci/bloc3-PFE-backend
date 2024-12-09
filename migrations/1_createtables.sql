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
    business_activity TEXT,
    nb_workers INTEGER,
    revenue DOUBLE PRECISION,
    labels TEXT[],
    dispute BOOLEAN DEFAULT FALSE,
    is_validated BOOLEAN DEFAULT FALSE,
    is_eligible BOOLEAN DEFAULT FALSE
);

CREATE TABLE IF NOT EXISTS pfe.template_company (
    company_id INTEGER REFERENCES pfe.companies(company_id),
    template_id INTEGER REFERENCES pfe.templates(template_id),
    PRIMARY KEY (company_id, template_id)
);

CREATE TABLE IF NOT EXISTS pfe.forms (
    form_id SERIAL PRIMARY KEY,
    company_id INTEGER REFERENCES pfe.companies(company_id),
    type VARCHAR(10) CHECK (type IN ('ODD', 'ESG'))
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
    question_status VARCHAR(255) NOT NULL,
    PRIMARY KEY (form_id, question_id)
);

CREATE TABLE IF NOT EXISTS pfe.choices_odd (
    choice_id SERIAL PRIMARY KEY,
    choice TEXT NOT NULL UNIQUE CHECK (choice IN (
        'Ne correspond pas à mes activités',
        'Pas de contribution',
        'Contribution occasionnelle',
        'Contribution générale',
        'Contribution spécifique',
        'Mission d''entreprise')),
    score DOUBLE PRECISION NOT NULL
);

CREATE TABLE IF NOT EXISTS pfe.answers_odd (
    answer_id SERIAL PRIMARY KEY,
    question_id INTEGER REFERENCES pfe.questions(question_id),
    choice_id INTEGER REFERENCES pfe.choices_odd(choice_id)
);

CREATE TABLE IF NOT EXISTS pfe.answers_esg (
    answer_id SERIAL PRIMARY KEY,
    question_id INTEGER REFERENCES pfe.questions(question_id),
    template TEXT NOT NULL,
    answer TEXT,
    score_now DOUBLE PRECISION NOT NULL,
    score_commitment_pact DOUBLE PRECISION,
    is_forced_engagement BOOLEAN NOT NULL
);

CREATE TABLE IF NOT EXISTS pfe.user_answer_esg (
    answer_id INTEGER REFERENCES pfe.answers_esg(answer_id),
    user_id INTEGER REFERENCES pfe.users(user_id),
    form_id INTEGER REFERENCES pfe.forms(form_id),
    answer TEXT,
    now BOOLEAN NOT NULL,
    commitment_pact BOOLEAN NOT NULL,
    comment TEXT,
    now_verif BOOLEAN ,
    commitment_pact_verif BOOLEAN,
    PRIMARY KEY (answer_id, user_id, form_id)
);

