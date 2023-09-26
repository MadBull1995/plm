-- This file should undo anything in `up.sql`
-- Drop the new indices
DROP INDEX IF EXISTS idx_unique_libraries;
DROP INDEX IF EXISTS idx_unique_libraries_no_org;

-- Re-create the old indices
CREATE UNIQUE INDEX idx_unique_libraries
ON libraries (name, version, org_id)
WHERE org_id IS NOT NULL;

CREATE UNIQUE INDEX idx_unique_libraries_no_org
ON libraries (name, version)
WHERE org_id IS NULL;
