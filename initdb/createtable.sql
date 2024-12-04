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

CREATE TABLE IF NOT EXISTS pfe.forms (
    form_id SERIAL PRIMARY KEY,
    company INTEGER REFERENCES companies(company_id),
    type VARCHAR(10) CHECK (type IN ('ODD', 'ESG')),
    nb_questions INTEGER,
    template TEXT
);

CREATE TABLE IF NOT EXISTS pfe.questions_form (
    form_id INTEGER REFERENCES forms(form_id),
    question_id INTEGER REFERENCES questions(id),
    PRIMARY KEY (form_id, question_id)
)
