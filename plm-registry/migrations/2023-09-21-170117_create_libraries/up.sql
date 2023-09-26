-- Your SQL goes here
CREATE TABLE libraries (
    lib_id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    version VARCHAR(50) NOT NULL,
    org_id INT REFERENCES organizations(org_id),
    public BOOLEAN NOT NULL,
    created_at TIMESTAMPTZ DEFAULT current_timestamp,
    updated_at TIMESTAMPTZ DEFAULT current_timestamp,
    UNIQUE (name, version, org_id)
);