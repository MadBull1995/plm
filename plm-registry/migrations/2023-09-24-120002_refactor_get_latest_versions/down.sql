-- This file should undo anything in `up.sql`
-- Your SQL goes here
-- Your SQL goes here
CREATE OR REPLACE FUNCTION get_latest_version(lib_name text)
RETURNS text AS $$
DECLARE
    rec         record;
    max_version text;
    v1 int;
    v2 int;
    v3 int;
    max_v1 int := 0;
    max_v2 int := 0;
    max_v3 int := 0;
BEGIN
    FOR rec IN SELECT version_id, version_number FROM versions
               INNER JOIN libraries ON versions.library_id = libraries.lib_id
               WHERE libraries.name = lib_name LOOP
        -- Assuming the version is in the format 'x.y.z'
        SELECT INTO v1, v2, v3
            CAST(split_part(rec.version_number, '.', 1) AS int),
            CAST(split_part(rec.version_number, '.', 2) AS int),
            CAST(split_part(rec.version_number, '.', 3) AS int);

        IF v1 > max_v1 OR (v1 = max_v1 AND v2 > max_v2) OR (v1 = max_v1 AND v2 = max_v2 AND v3 > max_v3) THEN
            max_version := rec.version_number;
            max_v1 := v1;
            max_v2 := v2;
            max_v3 := v3;
        END IF;
    END LOOP;

    RETURN max_version;
END;
$$ LANGUAGE plpgsql;
