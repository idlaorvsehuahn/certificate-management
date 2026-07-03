CREATE TABLE inventory_certificates (
    id UUID PRIMARY KEY,
    subject TEXT NOT NULL,
    issuer TEXT NOT NULL,
    status TEXT NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    CONSTRAINT inventory_certificates_status_check CHECK (status IN ('ACTIVE', 'REVOKED', 'EXPIRED'))
);

CREATE INDEX idx_inventory_certificates_status ON inventory_certificates(status);
CREATE INDEX idx_inventory_certificates_expires_at ON inventory_certificates(expires_at);
CREATE INDEX idx_inventory_certificates_subject ON inventory_certificates(subject);
