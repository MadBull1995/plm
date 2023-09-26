-- Your SQL goes here
-- Create versions table
CREATE TABLE versions (
    id SERIAL PRIMARY KEY,
    library_id INT NOT NULL REFERENCES libraries(lib_id),
    version_number VARCHAR(50) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    UNIQUE(library_id, version_number)
);

-- Create dependencies table
CREATE TABLE dependencies (
    id SERIAL PRIMARY KEY,
    version_id INT NOT NULL REFERENCES versions(id),
    dependent_version_id INT NOT NULL REFERENCES versions(id),
    dependency_range VARCHAR(50) NOT NULL,
    UNIQUE(version_id, dependent_version_id)
);
