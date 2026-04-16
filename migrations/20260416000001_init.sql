-- Table de health check pour valider la connexion Postgres
CREATE TABLE IF NOT EXISTS health_check (
    id SERIAL PRIMARY KEY,
    checked_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_health_check_checked_at
    ON health_check (checked_at DESC);
