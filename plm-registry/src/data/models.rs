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

use chrono::NaiveDateTime;
use diesel::sql_types::*;

use diesel::prelude::*;

#[derive(Insertable)]
#[diesel(table_name = crate::data::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser<'a> {
    pub username: &'a str,
    // pub email: &'a str,
    pub password_hash: &'a str,
    // pub created_at: NaiveDateTime,
    // pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::data::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub user_id: i32,
    pub username: String,
    pub email: Option<String>,
    pub password_hash: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Queryable, Identifiable)]
#[diesel(primary_key(org_id))]
#[diesel(table_name = crate::data::schema::organizations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Organization {
    pub org_id: i32,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(belongs_to(Organization, foreign_key = org_id))]
#[diesel(primary_key(user_id, org_id))]
#[diesel(table_name = crate::data::schema::user_organizations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserOrganization {
    pub user_id: i32,
    pub org_id: i32,
    pub role: i32,
}

#[derive(Debug, Selectable, Queryable, Identifiable)]
#[diesel(primary_key(lib_id))]
#[diesel(table_name = crate::data::schema::libraries)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Library {
    pub lib_id: i32,
    pub name: String,
    pub org_id: Option<i32>,
    pub public: bool,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::data::schema::libraries)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewLibrary<'a> {
    pub name: &'a str,
    pub org_id: Option<&'a i32>,
    pub public: bool,
}

#[derive(Debug, QueryableByName)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct LatestVersion {
    #[diesel(sql_type = Integer)]
    pub max_version_id: i32,
    #[diesel(sql_type = Text)]
    pub max_version_number: String,
}

// In your models.rs or a similar file
#[derive(Queryable, Debug)]
#[diesel(table_name = crate::data::schema::versions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Version {
    pub id: i32,
    pub library_id: i32,
    pub version_number: String,
    pub created_at: NaiveDateTime,
}

#[derive(Queryable, Associations)]
#[diesel(belongs_to(Version, foreign_key = version_id))]
#[diesel(table_name = crate::data::schema::dependencies)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Dependency {
    pub id: i32,
    pub version_id: i32,
    pub dependent_version_id: i32,
    pub dependency_range: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::data::schema::versions)]
pub struct NewVersion<'a> {
    pub library_id: i32,
    pub version_number: &'a str,
}

#[derive(Insertable)]
#[diesel(table_name = crate::data::schema::dependencies)]
pub struct NewDependency<'a> {
    pub version_id: i32,
    pub dependent_version_id: i32,
    pub dependency_range: &'a str,
}
