DO $$
BEGIN
    IF NOT EXISTS (SELECT FROM pg_database WHERE datname = 'pfe_backend') THEN
        CREATE DATABASE pfe_backend;
    END IF;
END $$;
