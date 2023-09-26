// @generated automatically by Diesel CLI.

diesel::table! {
    dependencies (id) {
        id -> Int4,
        version_id -> Int4,
        dependent_version_id -> Int4,
        #[max_length = 50]
        dependency_range -> Varchar,
    }
}

diesel::table! {
    libraries (lib_id) {
        lib_id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        org_id -> Nullable<Int4>,
        public -> Bool,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    organizations (org_id) {
        org_id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    user_organizations (user_id, org_id) {
        user_id -> Int4,
        org_id -> Int4,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Int4,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        email -> Nullable<Varchar>,
        #[max_length = 255]
        password_hash -> Varchar,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    versions (id) {
        id -> Int4,
        library_id -> Int4,
        #[max_length = 50]
        version_number -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::joinable!(libraries -> organizations (org_id));
diesel::joinable!(user_organizations -> organizations (org_id));
diesel::joinable!(user_organizations -> users (user_id));
diesel::joinable!(versions -> libraries (library_id));

diesel::allow_tables_to_appear_in_same_query!(
    dependencies,
    libraries,
    organizations,
    user_organizations,
    users,
    versions,
);
