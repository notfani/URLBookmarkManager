use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;
use crate::db;
use crate::model::{Bookmark, NewBookmark, UpdateBookmark, Category, NewCategory, CreateCategoryRequest, BookmarkWithCategory, RegisterRequest, LoginRequest, NewUser, User, AuthResponse};
use crate::auth;
use crate::middleware::AuthenticatedUser;

#[derive(Serialize, Deserialize)]
pub struct CreateBookmarkRequest {
    pub title: String,
    pub url: String,
    pub description: Option<String>,
    pub category_id: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateBookmarkRequest {
    pub title: Option<String>,
    pub url: Option<String>,
    pub description: Option<String>,
    pub category_id: Option<Uuid>,
}

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: None,
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            message: Some(message),
        }
    }
}

pub async fn create_bookmark(
    conn: web::Data<db::Pool>,
    item: web::Json<CreateBookmarkRequest>,
    auth_user: AuthenticatedUser,
) -> Result<HttpResponse> {
    let mut conn = conn.get().expect("Failed to get DB connection");

    let new_bookmark = NewBookmark {
        title: &item.title,
        url: &item.url,
        description: item.description.as_deref(),
        category_id: item.category_id,
        user_id: auth_user.user_id,
    };

    match db::create_bookmark(&mut conn, new_bookmark) {
        Ok(bookmark) => Ok(HttpResponse::Ok().json(ApiResponse::success(bookmark))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<Bookmark>::error(e.to_string()))),
    }
}

pub async fn get_all_bookmarks(conn: web::Data<db::Pool>) -> Result<HttpResponse> {
    let mut conn = conn.get().expect("Failed to get DB connection");

    match db::get_all_bookmarks(&mut conn) {
        Ok(bookmarks) => Ok(HttpResponse::Ok().json(ApiResponse::success(bookmarks))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<Vec<BookmarkWithCategory>>::error(e.to_string()))),
    }
}

pub async fn get_bookmark_by_id(
    conn: web::Data<db::Pool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let mut conn = conn.get().expect("Failed to get DB connection");
    let id = path.into_inner();

    match db::get_bookmark_by_id(&mut conn, &id) {
        Ok(bookmark) => Ok(HttpResponse::Ok().json(ApiResponse::success(bookmark))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<BookmarkWithCategory>::error(e.to_string()))),
    }
}

pub async fn update_bookmark(
    conn: web::Data<db::Pool>,
    path: web::Path<Uuid>,
    item: web::Json<UpdateBookmarkRequest>,
) -> Result<HttpResponse> {
    let mut conn = conn.get().expect("Failed to get DB connection");
    let id = path.into_inner();

    let update_data = UpdateBookmark {
        title: item.title.as_deref(),
        url: item.url.as_deref(),
        description: item.description.as_deref(),
        category_id: item.category_id,
    };

    match db::update_bookmark(&mut conn, &id, update_data) {
        Ok(bookmark) => Ok(HttpResponse::Ok().json(ApiResponse::success(bookmark))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<Bookmark>::error(e.to_string()))),
    }
}

pub async fn delete_bookmark(
    conn: web::Data<db::Pool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let mut conn = conn.get().expect("Failed to get DB connection");
    let id = path.into_inner();

    match db::delete_bookmark(&mut conn, &id) {
        Ok(_) => Ok(HttpResponse::Ok().json(ApiResponse::success("Bookmark deleted"))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<&str>::error(e.to_string()))),
    }
}

pub async fn search_bookmarks(
    conn: web::Data<db::Pool>,
    query: web::Query<HashMap<String, String>>,
) -> Result<HttpResponse> {
    let mut conn = conn.get().expect("Failed to get DB connection");
    let empty_string = String::new();
    let search_query = query.get("q").unwrap_or(&empty_string);

    match db::search_bookmarks(&mut conn, search_query) {
        Ok(bookmarks) => Ok(HttpResponse::Ok().json(ApiResponse::success(bookmarks))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<Vec<BookmarkWithCategory>>::error(e.to_string()))),
    }
}

pub async fn get_bookmarks_by_category(
    conn: web::Data<db::Pool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let mut conn = conn.get().expect("Failed to get DB connection");
    let category_id = path.into_inner();

    match db::get_bookmarks_by_category(&mut conn, &category_id) {
        Ok(bookmarks) => Ok(HttpResponse::Ok().json(ApiResponse::success(bookmarks))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<Vec<BookmarkWithCategory>>::error(e.to_string()))),
    }
}

pub async fn register(
    conn: web::Data<db::Pool>,
    req: web::Json<RegisterRequest>,
) -> Result<HttpResponse> {
    let mut conn = conn.get().expect("Failed to get DB connection");

    if db::find_user_by_username(&mut conn, &req.username).is_ok() {
        return Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error("Username already exists".to_string())));
    }

    if db::find_user_by_email(&mut conn, &req.email).is_ok() {
        return Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error("Email already exists".to_string())));
    }

    let password_hash = bcrypt::hash(&req.password, bcrypt::DEFAULT_COST)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to hash password"))?;

    let new_user = NewUser {
        username: req.username.clone(),
        email: req.email.clone(),
        password_hash,
        full_name: req.full_name.clone(),
    };

    match db::create_user(&mut conn, new_user) {
        Ok(user) => {
            let token = auth::create_jwt(&user.id, &user.username)
                .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to create token"))?;

            let auth_response = AuthResponse { token, user };
            Ok(HttpResponse::Ok().json(ApiResponse::success(auth_response)))
        }
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(e.to_string()))),
    }
}

pub async fn login(
    conn: web::Data<db::Pool>,
    req: web::Json<LoginRequest>,
) -> Result<HttpResponse> {
    let mut conn = conn.get().expect("Failed to get DB connection");

    match db::find_user_by_username(&mut conn, &req.username) {
        Ok(user) => {
            let password_valid = bcrypt::verify(&req.password, &user.password_hash)
                .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to verify password"))?;

            if !password_valid {
                return Ok(HttpResponse::Unauthorized().json(ApiResponse::<()>::error("Invalid credentials".to_string())));
            }

            let token = auth::create_jwt(&user.id, &user.username)
                .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to create token"))?;

            let auth_response = AuthResponse { token, user };
            Ok(HttpResponse::Ok().json(ApiResponse::success(auth_response)))
        }
        Err(_) => Ok(HttpResponse::Unauthorized().json(ApiResponse::<()>::error("Invalid credentials".to_string()))),
    }
}

pub async fn create_category(
    conn: web::Data<db::Pool>,
    item: web::Json<CreateCategoryRequest>,
    auth_user: AuthenticatedUser,
) -> Result<HttpResponse> {
    let mut conn = conn.get().expect("Failed to get DB connection");

    let new_category = NewCategory {
        name: &item.name,
        user_id: auth_user.user_id,
    };

    match db::create_category(&mut conn, new_category) {
        Ok(category) => Ok(HttpResponse::Ok().json(ApiResponse::success(category))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<Category>::error(e.to_string()))),
    }
}

pub async fn get_all_categories(
    conn: web::Data<db::Pool>,
    auth_user: AuthenticatedUser,
) -> Result<HttpResponse> {
    let mut conn = conn.get().expect("Failed to get DB connection");

    match db::get_user_categories(&mut conn, &auth_user.user_id) {
        Ok(categories) => Ok(HttpResponse::Ok().json(ApiResponse::success(categories))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<Vec<Category>>::error(e.to_string()))),
    }
}

pub async fn get_current_user(
    conn: web::Data<db::Pool>,
    auth_user: AuthenticatedUser,
) -> Result<HttpResponse> {
    let mut conn = conn.get().expect("Failed to get DB connection");

    match db::get_user_by_id(&mut conn, &auth_user.user_id) {
        Ok(user) => Ok(HttpResponse::Ok().json(ApiResponse::success(user))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<User>::error(e.to_string()))),
    }
}
