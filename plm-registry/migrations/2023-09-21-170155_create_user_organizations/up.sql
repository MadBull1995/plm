-- Your SQL goes here
 CREATE TABLE user_organizations (
    user_id INT REFERENCES users(user_id),
    org_id INT REFERENCES organizations(org_id),
    PRIMARY KEY (user_id, org_id)
);