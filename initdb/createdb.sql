DO $$
BEGIN
    IF NOT EXISTS (SELECT FROM pg_database WHERE datname = 'pfe_db') THEN
        CREATE DATABASE pfe_db;
    END IF;
END $$;
