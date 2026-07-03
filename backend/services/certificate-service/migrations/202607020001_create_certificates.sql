CREATE TABLE certificates (
    id UUID PRIMARY KEY,
    subject TEXT NOT NULL,
    issuer TEXT NOT NULL,
    serial_number TEXT NOT NULL UNIQUE,
    expiration TIMESTAMPTZ NOT NULL,
    issued_at TIMESTAMPTZ NOT NULL,
    status TEXT NOT NULL,
    pem TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT certificates_status_check CHECK (status IN ('ACTIVE', 'REVOKED', 'EXPIRED'))
);

CREATE TABLE certificate_sans (
    id BIGSERIAL PRIMARY KEY,
    certificate_id UUID NOT NULL REFERENCES certificates(id) ON DELETE CASCADE,
    dns_name TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (certificate_id, dns_name)
);

CREATE INDEX idx_certificates_status ON certificates(status);
CREATE INDEX idx_certificates_expiration ON certificates(expiration);
CREATE INDEX idx_certificates_subject ON certificates(subject);
CREATE INDEX idx_certificate_sans_dns_name ON certificate_sans(dns_name);
