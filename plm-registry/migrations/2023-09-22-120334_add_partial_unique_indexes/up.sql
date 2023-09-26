-- Your SQL goes here
-- Create a unique index where org_id is NOT NULL
CREATE UNIQUE INDEX idx_unique_libraries
ON libraries (name, version, org_id)
WHERE org_id IS NOT NULL;

-- Create a unique index where org_id is NULL
CREATE UNIQUE INDEX idx_unique_libraries_no_org
ON libraries (name, version)
WHERE org_id IS NULL;
