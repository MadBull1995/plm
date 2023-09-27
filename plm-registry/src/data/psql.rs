// Copyright 2023 Sylk Technologies
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::{diesel_migrations, models::Library};
use diesel::prelude::*;
use diesel::{pg::PgConnection, sql_types::Text};
use diesel::sql_query;
use std::ops::DerefMut;
use tokio::sync::Mutex;
use tracing::debug;

use dotenvy::dotenv;
use std::env;
use std::sync::Arc;

use crate::models::{
    Dependency, LatestVersion, NewDependency, NewLibrary, NewUser, NewVersion, User, Version,
};

type QueryResult<T> = Result<T, diesel::result::Error>;
type DB = diesel::pg::Pg;
const MIGRATIONS: diesel_migrations::EmbeddedMigrations = embed_migrations!();

#[derive(Clone)]
pub struct QueryLayer {
    pub conn: Arc<Mutex<PgConnection>>,
}

impl Default for QueryLayer {
    fn default() -> Self {
        Self::new()
    }
}

impl QueryLayer {
    pub fn new() -> Self {
        let mut conn = establish_connection();
        initialize_schema(&mut conn);

        Self {
            conn: Arc::new(Mutex::new(conn)),
        }
    }

    // Users queries

    pub async fn get_user(&self, user_name: &str) -> QueryResult<Option<User>> {
        use crate::schema::users::dsl::*;
        let mut c = self.conn.lock().await;

        users
            .filter(username.eq(user_name))
            .select(User::as_select())
            .first(c.deref_mut())
            .optional() // This allows for returning an Option<Post>, otherwise it will throw an error
    }

    pub async fn create_user(&self, user: &plm_core::CreateUserRequest) -> QueryResult<User> {
        let new_user = NewUser {
            username: &user.username,
            password_hash: &user.password,
        };
        let mut c = self.conn.lock().await;
        diesel::insert_into(crate::schema::users::table)
            .values(&new_user)
            .returning(User::as_returning())
            .get_result(c.deref_mut())
    }

    pub async fn find_user_by_id(&self, user_id: i32) -> QueryResult<User> {
        // Use Diesel to find a user by id
        let mut c = self.conn.lock().await;
        crate::schema::users::table
            .find(user_id)
            .first(c.deref_mut())
    }

    // Libraries queries

    /// Create a new version entry
    pub fn create_version(
        &self,
        new_version: &NewVersion<'_>,
        conn: &mut PgConnection,
    ) -> QueryResult<Version> {
        use crate::schema::versions;
        diesel::insert_into(versions::table)
            .values(new_version)
            .returning((
                versions::id,
                versions::library_id,
                versions::version_number,
                versions::created_at,
            ))
            .get_result(conn)
    }

    /// Retrieve all versions for a specific library
    pub async fn get_versions_by_library(&self, lib_id: i32) -> QueryResult<Vec<Version>> {
        use crate::schema::versions;

        let mut c = self.conn.lock().await;
        versions::table
            .filter(versions::library_id.eq(lib_id))
            .load::<Version>(c.deref_mut())
    }

    /// Create a new dependency entry
    pub fn create_dependency(
        &self,
        new_dependency: &NewDependency<'_>,
        conn: &mut PgConnection,
    ) -> QueryResult<Dependency> {
        use crate::schema::dependencies;

        diesel::insert_into(dependencies::table)
            .values(new_dependency)
            .returning((
                dependencies::id,
                dependencies::version_id,
                dependencies::dependent_version_id,
                dependencies::dependency_range,
            ))
            .get_result(conn)
    }

    /// Retrieve dependencies for a specific version
    pub fn get_dependencies_by_version(
        &self,
        ver_id: i32,
        conn: &mut PgConnection,
    ) -> QueryResult<Vec<Dependency>> {
        use crate::schema::dependencies;

        dependencies::table
            .filter(dependencies::version_id.eq(ver_id))
            .load::<Dependency>(conn)
    }

    /// Retrieve dependent versions for a specific version
    pub fn get_dependent_versions(
        &self,
        ver_id: i32,
        conn: &mut PgConnection,
    ) -> QueryResult<Vec<Dependency>> {
        use crate::schema::dependencies;

        dependencies::table
            .filter(dependencies::dependent_version_id.eq(ver_id))
            .load::<Dependency>(conn)
    }

    pub fn create_release(
        &self,
        release: &plm_core::Library,
        conn: &mut PgConnection,
    ) -> QueryResult<Library> {
        let new_release = NewLibrary {
            name: &release.name,
            org_id: None,
            public: false,
        };
        // let mut c = self.conn.lock().await;
        diesel::insert_into(crate::schema::libraries::table)
            .values(&new_release)
            .returning(Library::as_returning())
            .get_result(conn)
    }

    pub async fn get_latest_version_for_lib(
        &self,
        lib_name: &str,
    ) -> QueryResult<Option<LatestVersion>> {
        let mut c = self.conn.lock().await;

        let result = sql_query("SELECT * FROM get_latest_version($1)")
            .bind::<Text, _>(lib_name)
            .load::<LatestVersion>(c.deref_mut())?
            .pop();
        println!("{:?}", result);

        // let result = sql_query("")
        // .bind::<Text, _>(lib_name)
        // .get_result(c.deref_mut());
        match result {
            None => Err(diesel::NotFound),
            Some(v) => Ok(Some(v)),
        }
    }

    pub async fn get_release(
        &self,
        lib_name: &str,
        _lib_version: Option<i32>,
        _lib_scope: Option<String>,
    ) -> QueryResult<Option<(Library, Vec<Version>)>> {
        use crate::schema::{libraries::dsl::*, versions};
        let mut c = self.conn.lock().await;
        debug!("{:?}", lib_name);
        // Start with a base query for libraries
        let query = libraries.filter(name.eq(lib_name)).into_boxed(); // Boxed queries allow dynamic composition

        // // Conditionally apply optional filters
        // if let Some(v) = lib_version {
        //     query = query.filter(libraries::version.eq(v));
        // }

        // Execute the query for libraries
        if let Some(library) = query
            .select(Library::as_select())
            .first::<Library>(c.deref_mut())
            .optional()?
        {
            println!("{:?}", library);
            // Now query for versions and dependencies based on the found library
            let related_versions = versions::table
                .filter(versions::library_id.eq(library.lib_id))
                .load::<Version>(c.deref_mut())?;
            println!("{:?}", related_versions);
            // let related_dependencies = Dependency::belonging_to(&related_versions)
            //     .load::<Dependency>(c.deref_mut())?
            //     .grouped_by(&related_versions);

            Ok(Some((library, related_versions)))
        } else {
            Ok(None)
        }
    }
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|e| panic!("Error connecting to {}: {:?}", database_url, e))
}

pub fn initialize_schema(conn: &mut impl diesel_migrations::MigrationHarness<DB>) {
    conn.run_pending_migrations(MIGRATIONS)
        .expect("failed to run pending migrations");

    // let statements = vec![
    //     r#"
    //     CREATE TABLE IF NOT EXISTS users (
    //         user_id SERIAL PRIMARY KEY,
    //         username VARCHAR(255) UNIQUE NOT NULL,
    //         email VARCHAR(255) UNIQUE NOT NULL,
    //         password_hash VARCHAR(255) NOT NULL,
    //         created_at TIMESTAMPTZ DEFAULT current_timestamp,
    //         updated_at TIMESTAMPTZ DEFAULT current_timestamp
    //     );
    //     "#,
    //     r#"
    //     CREATE TABLE IF NOT EXISTS organizations (
    //         org_id SERIAL PRIMARY KEY,
    //         name VARCHAR(255) UNIQUE NOT NULL,
    //         created_at TIMESTAMPTZ DEFAULT current_timestamp,
    //         updated_at TIMESTAMPTZ DEFAULT current_timestamp
    //     );
    //     "#,
    //     r#"
    //     CREATE TABLE IF NOT EXISTS user_organizations (
    //         user_id INT REFERENCES users(user_id),
    //         org_id INT REFERENCES organizations(org_id),
    //         PRIMARY KEY (user_id, org_id)
    //     );
    //     "#,
    //     r#"
    //     CREATE TABLE IF NOT EXISTS libraries (
    //         lib_id SERIAL PRIMARY KEY,
    //         name VARCHAR(255) NOT NULL,
    //         version VARCHAR(50) NOT NULL,
    //         org_id INT REFERENCES organizations(org_id),
    //         public BOOLEAN NOT NULL,
    //         created_at TIMESTAMPTZ DEFAULT current_timestamp,
    //         updated_at TIMESTAMPTZ DEFAULT current_timestamp,
    //         UNIQUE (name, version, org_id)
    //     );
    //     "#,
    // ];
    // for stmt in statements {
    //     diesel::sql_query(stmt)
    //         .execute(conn)
    //         .expect("failed to init schema");
    // }
}
