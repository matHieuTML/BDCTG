CREATE TABLE IF NOT EXISTS inscriptions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    activity_slug TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(user_id, activity_slug)
);

CREATE INDEX idx_inscriptions_user ON inscriptions(user_id);
CREATE INDEX idx_inscriptions_slug ON inscriptions(activity_slug);
