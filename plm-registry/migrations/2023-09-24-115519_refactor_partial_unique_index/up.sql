-- Your SQL goes here
DROP INDEX IF EXISTS idx_unique_libraries;
DROP INDEX IF EXISTS idx_unique_libraries_no_org;

-- Create the new indices
CREATE UNIQUE INDEX idx_unique_libraries
ON libraries (name, org_id)
WHERE org_id IS NOT NULL;

CREATE UNIQUE INDEX idx_unique_libraries_no_org
ON libraries (name)
WHERE org_id IS NULL;
