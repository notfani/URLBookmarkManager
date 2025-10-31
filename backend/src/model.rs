use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::NaiveDateTime;

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = crate::schema::categories)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Category {
    pub id: Uuid,
    pub name: String,
    pub user_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::categories)]
pub struct NewCategory<'a> {
    pub name: &'a str,
    pub user_id: Uuid,
}

#[derive(Deserialize)]
pub struct CreateCategoryRequest {
    pub name: String,
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = crate::schema::bookmarks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Bookmark {
    pub id: Uuid,
    pub title: String,
    pub url: String,
    pub description: Option<String>,
    pub category_id: Uuid,
    pub user_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::bookmarks)]
pub struct NewBookmark<'a> {
    pub title: &'a str,
    pub url: &'a str,
    pub description: Option<&'a str>,
    pub category_id: Uuid,
    pub user_id: Uuid,
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = crate::schema::bookmarks)]
pub struct UpdateBookmark<'a> {
    pub title: Option<&'a str>,
    pub url: Option<&'a str>,
    pub description: Option<&'a str>,
    pub category_id: Option<Uuid>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BookmarkWithCategory {
    pub id: Uuid,
    pub title: String,
    pub url: String,
    pub description: Option<String>,
    pub category: Category,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Bookmark {
    pub fn with_category(self, category: Category) -> BookmarkWithCategory {
        BookmarkWithCategory {
            id: self.id,
            title: self.title,
            url: self.url,
            description: self.description,
            category,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub full_name: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub full_name: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub full_name: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: User,
}
