
use actix_web::{test, web, App, http::header};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use uuid::Uuid;
use crate::model::{Bookmark, NewBookmark, Category, NewCategory, NewUser, User, RegisterRequest, LoginRequest};
use crate::db;
use crate::handlers::*;
use crate::auth;

type Pool = r2d2::Pool<ConnectionManager<diesel::PgConnection>>;

fn setup_test_db() -> Pool {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost/bookmark_test".to_string());

    let manager = ConnectionManager::<diesel::PgConnection>::new(&database_url);
    r2d2::Pool::builder()
        .max_size(5)
        .build(manager)
        .expect("Failed to create test pool")
}

fn create_test_user(conn: &mut PgConnection) -> User {
    let password_hash = bcrypt::hash("testpassword", bcrypt::DEFAULT_COST).unwrap();
    let new_user = NewUser {
        username: format!("testuser_{}", Uuid::new_v4()),
        email: format!("test_{}@example.com", Uuid::new_v4()),
        password_hash,
        full_name: Some("Test User".to_string()),
    };
    db::create_user(conn, new_user).unwrap()
}

fn create_test_category(conn: &mut PgConnection, user: &User) -> Category {
    let new_category = NewCategory {
        name: "Test Category",
        user_id: user.id,
    };
    db::create_category(conn, new_category).unwrap()
}

#[tokio::test]
async fn test_find_user_by_email() {
    let pool = setup_test_db();
    let mut conn = pool.get().unwrap();

    let user = create_test_user(&mut conn);
    let found_user = db::find_user_by_email(&mut conn, &user.email);

    assert!(found_user.is_ok());
    assert_eq!(found_user.unwrap().id, user.id);
}

#[actix_web::test]
async fn test_create_user() {
    let pool = setup_test_db();
    let mut conn = pool.get().unwrap();

    let password_hash = bcrypt::hash("password123", bcrypt::DEFAULT_COST).unwrap();
    let new_user = NewUser {
        username: format!("newuser_{}", Uuid::new_v4()),
        email: format!("new_{}@example.com", Uuid::new_v4()),
        password_hash: password_hash.clone(),
        full_name: Some("New User".to_string()),
    };

    let result = db::create_user(&mut conn, new_user);
    assert!(result.is_ok());

    let user = result.unwrap();
    assert_eq!(user.password_hash, password_hash);
}

#[actix_web::test]
async fn test_find_user_by_username() {
    let pool = setup_test_db();
    let mut conn = pool.get().unwrap();

    let user = create_test_user(&mut conn);
    let found_user = db::find_user_by_username(&mut conn, &user.username);

    assert!(found_user.is_ok());
    assert_eq!(found_user.unwrap().id, user.id);
}

#[tokio::test] async fn test_get_user_by_id() {
    let pool = setup_test_db();
    let mut conn = pool.get().unwrap();

    let user = create_test_user(&mut conn);
    let found_user = db::get_user_by_id(&mut conn, &user.id);

    assert!(found_user.is_ok());
    assert_eq!(found_user.unwrap().username, user.username);
}

#[tokio::test] async fn test_create_category() {
    let pool = setup_test_db();
    let mut conn = pool.get().unwrap();
    let user = create_test_user(&mut conn);

    let new_category = NewCategory {
        name: "Work",
        user_id: user.id,
    };

    let result = db::create_category(&mut conn, new_category);
    assert!(result.is_ok());

    let category = result.unwrap();
    assert_eq!(category.name, "Work");
    assert_eq!(category.user_id, user.id);
}

#[tokio::test] async fn test_get_all_categories() {
    let pool = setup_test_db();
    let mut conn = pool.get().unwrap();
    let user = create_test_user(&mut conn);

    create_test_category(&mut conn, &user);
    create_test_category(&mut conn, &user);

    let result = db::get_all_categories(&mut conn);
    assert!(result.is_ok());
    assert!(result.unwrap().len() >= 2);
}

#[tokio::test] async fn test_get_user_categories() {
    let pool = setup_test_db();
    let mut conn = pool.get().unwrap();
    let user = create_test_user(&mut conn);

    create_test_category(&mut conn, &user);
    create_test_category(&mut conn, &user);

    let result = db::get_user_categories(&mut conn, &user.id);
    assert!(result.is_ok());
    assert!(result.unwrap().len() >= 2);
}

#[tokio::test] async fn test_get_category_by_id() {
    let pool = setup_test_db();
    let mut conn = pool.get().unwrap();
    let user = create_test_user(&mut conn);

    let category = create_test_category(&mut conn, &user);
    let result = db::get_category_by_id(&mut conn, &category.id);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().id, category.id);
}

#[tokio::test] async fn test_create_bookmark() {
    let pool = setup_test_db();
    let mut conn = pool.get().unwrap();
    let user = create_test_user(&mut conn);
    let category = create_test_category(&mut conn, &user);

    let new_bookmark = NewBookmark {
        title: "Test Bookmark",
        url: "https://example.com",
        description: Some("Test description"),
        category_id: category.id,
        user_id: user.id,
    };

    let result = db::create_bookmark(&mut conn, new_bookmark);
    assert!(result.is_ok());

    let bookmark = result.unwrap();
    assert_eq!(bookmark.title, "Test Bookmark");
    assert_eq!(bookmark.url, "https://example.com");
    assert_eq!(bookmark.category_id, category.id);
}

#[tokio::test] async fn test_get_all_bookmarks() {
    let pool = setup_test_db();
    let mut conn = pool.get().unwrap();
    let user = create_test_user(&mut conn);
    let category = create_test_category(&mut conn, &user);

    let new_bookmark = NewBookmark {
        title: "Bookmark 1",
        url: "https://example1.com",
        description: None,
        category_id: category.id,
        user_id: user.id,
    };
    db::create_bookmark(&mut conn, new_bookmark).unwrap();

    let result = db::get_all_bookmarks(&mut conn);
    assert!(result.is_ok());
    assert!(result.unwrap().len() > 0);
}

#[tokio::test] async fn test_get_bookmark_by_id() {
    let pool = setup_test_db();
    let mut conn = pool.get().unwrap();
    let user = create_test_user(&mut conn);
    let category = create_test_category(&mut conn, &user);

    let new_bookmark = NewBookmark {
        title: "Find Me",
        url: "https://findme.com",
        description: Some("Find this bookmark"),
        category_id: category.id,
        user_id: user.id,
    };
    let bookmark = db::create_bookmark(&mut conn, new_bookmark).unwrap();

    let result = db::get_bookmark_by_id(&mut conn, &bookmark.id);
    assert!(result.is_ok());

    let found = result.unwrap();
    assert_eq!(found.title, "Find Me");
    assert_eq!(found.category_name, "Test Category");
}

#[tokio::test] async fn test_update_bookmark() {
    let pool = setup_test_db();
    let mut conn = pool.get().unwrap();
    let user = create_test_user(&mut conn);
    let category = create_test_category(&mut conn, &user);

    let new_bookmark = NewBookmark {
        title: "Original Title",
        url: "https://original.com",
        description: Some("Original description"),
        category_id: category.id,
        user_id: user.id,
    };
    let bookmark = db::create_bookmark(&mut conn, new_bookmark).unwrap();

    let update_data = crate::model::UpdateBookmark {
        title: Some("Updated Title"),
        url: Some("https://updated.com"),
        description: Some("Updated description"),
        category_id: Some(category.id),
    };

    let result = db::update_bookmark(&mut conn, &bookmark.id, update_data);
    assert!(result.is_ok());

    let updated = result.unwrap();
    assert_eq!(updated.title, "Updated Title");
    assert_eq!(updated.url, "https://updated.com");
}

#[tokio::test] async fn test_delete_bookmark() {
    let pool = setup_test_db();
    let mut conn = pool.get().unwrap();
    let user = create_test_user(&mut conn);
    let category = create_test_category(&mut conn, &user);

    let new_bookmark = NewBookmark {
        title: "Delete Me",
        url: "https://deleteme.com",
        description: None,
        category_id: category.id,
        user_id: user.id,
    };
    let bookmark = db::create_bookmark(&mut conn, new_bookmark).unwrap();

    let result = db::delete_bookmark(&mut conn, &bookmark.id);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1);

    let find_result = db::get_bookmark_by_id(&mut conn, &bookmark.id);
    assert!(find_result.is_err());
}

#[tokio::test] async fn test_search_bookmarks() {
    let pool = setup_test_db();
    let mut conn = pool.get().unwrap();
    let user = create_test_user(&mut conn);
    let category = create_test_category(&mut conn, &user);

    let bookmark1 = NewBookmark {
        title: "Rust Programming Language",
        url: "https://www.rust-lang.org",
        description: Some("Official Rust website"),
        category_id: category.id,
        user_id: user.id,
    };

    let bookmark2 = NewBookmark {
        title: "Google Search",
        url: "https://www.google.com",
        description: Some("Search engine"),
        category_id: category.id,
        user_id: user.id,
    };

    db::create_bookmark(&mut conn, bookmark1).unwrap();
    db::create_bookmark(&mut conn, bookmark2).unwrap();

    let results = db::search_bookmarks(&mut conn, "Rust").unwrap();
    assert!(results.iter().any(|b| b.title.contains("Rust")));

    let results = db::search_bookmarks(&mut conn, "google").unwrap();
    assert!(results.iter().any(|b| b.title.contains("Google")));
}

#[tokio::test] async fn test_get_bookmarks_by_category() {
    let pool = setup_test_db();
    let mut conn = pool.get().unwrap();
    let user = create_test_user(&mut conn);
    let category1 = create_test_category(&mut conn, &user);

    let new_category2 = NewCategory {
        name: "Category 2",
        user_id: user.id,
    };
    let category2 = db::create_category(&mut conn, new_category2).unwrap();

    let bookmark1 = NewBookmark {
        title: "Cat1 Bookmark",
        url: "https://cat1.com",
        description: None,
        category_id: category1.id,
        user_id: user.id,
    };

    let bookmark2 = NewBookmark {
        title: "Cat2 Bookmark",
        url: "https://cat2.com",
        description: None,
        category_id: category2.id,
        user_id: user.id,
    };

    db::create_bookmark(&mut conn, bookmark1).unwrap();
    db::create_bookmark(&mut conn, bookmark2).unwrap();

    let results = db::get_bookmarks_by_category(&mut conn, &category1.id).unwrap();
    assert!(results.iter().any(|b| b.title == "Cat1 Bookmark"));
    assert!(!results.iter().any(|b| b.title == "Cat2 Bookmark"));
}

#[tokio::test] async fn test_create_jwt() {
    let user_id = Uuid::new_v4();
    let username = "testuser";

    let result = auth::create_jwt(&user_id, username);
    assert!(result.is_ok());

    let token = result.unwrap();
    assert!(!token.is_empty());
}

#[tokio::test] async fn test_verify_jwt() {
    let user_id = Uuid::new_v4();
    let username = "testuser";

    let token = auth::create_jwt(&user_id, username).unwrap();
    let result = auth::verify_jwt(&token);

    assert!(result.is_ok());
    let claims = result.unwrap();
    assert_eq!(claims.username, username);
    assert_eq!(claims.sub, user_id.to_string());
}

#[tokio::test] async fn test_verify_invalid_jwt() {
    let result = auth::verify_jwt("invalid.token.here");
    assert!(result.is_err());
}

#[tokio::test] async fn test_expired_jwt() {
    let result = auth::verify_jwt("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.invalid.signature");
    assert!(result.is_err());
}

#[actix_web::test]
async fn test_register_handler() {
    let pool = setup_test_db();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/api/register", web::post().to(register))
    ).await;

    let req = test::TestRequest::post()
        .uri("/api/register")
        .set_json(&RegisterRequest {
            username: format!("newuser_{}", Uuid::new_v4()),
            email: format!("new_{}@example.com", Uuid::new_v4()),
            password: "password123".to_string(),
            full_name: Some("New User".to_string()),
        })
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_register_duplicate_username() {
    let pool = setup_test_db();
    let mut conn = pool.get().unwrap();
    let user = create_test_user(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/api/register", web::post().to(register))
    ).await;

    let req = test::TestRequest::post()
        .uri("/api/register")
        .set_json(&RegisterRequest {
            username: user.username.clone(),
            email: format!("different_{}@example.com", Uuid::new_v4()),
            password: "password123".to_string(),
            full_name: Some("Different User".to_string()),
        })
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 400);
}

#[actix_web::test]
async fn test_register_duplicate_email() {
    let pool = setup_test_db();
    let mut conn = pool.get().unwrap();
    let user = create_test_user(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/api/register", web::post().to(register))
    ).await;

    let req = test::TestRequest::post()
        .uri("/api/register")
        .set_json(&RegisterRequest {
            username: format!("different_user_{}", Uuid::new_v4()),
            email: user.email.clone(),
            password: "password123".to_string(),
            full_name: Some("Different User".to_string()),
        })
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 400);
}

#[actix_web::test]
async fn test_login_handler() {
    let pool = setup_test_db();
    let mut conn = pool.get().unwrap();

    let password_hash = bcrypt::hash("password123", bcrypt::DEFAULT_COST).unwrap();
    let new_user = NewUser {
        username: format!("loginuser_{}", Uuid::new_v4()),
        email: format!("login_{}@example.com", Uuid::new_v4()),
        password_hash,
        full_name: Some("Login User".to_string()),
    };
    let user = db::create_user(&mut conn, new_user).unwrap();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/api/login", web::post().to(login))
    ).await;

    let req = test::TestRequest::post()
        .uri("/api/login")
        .set_json(&LoginRequest {
            username: user.username.clone(),
            password: "password123".to_string(),
        })
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_login_invalid_credentials() {
    let pool = setup_test_db();
    let mut conn = pool.get().unwrap();
    let user = create_test_user(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/api/login", web::post().to(login))
    ).await;

    let req = test::TestRequest::post()
        .uri("/api/login")
        .set_json(&LoginRequest {
            username: user.username.clone(),
            password: "wrongpassword".to_string(),
        })
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 401);
}

#[actix_web::test]
async fn test_login_nonexistent_user() {
    let pool = setup_test_db();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/api/login", web::post().to(login))
    ).await;

    let req = test::TestRequest::post()
        .uri("/api/login")
        .set_json(&LoginRequest {
            username: "nonexistentuser".to_string(),
            password: "password123".to_string(),
        })
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 401);
}

#[actix_web::test]
async fn test_get_all_bookmarks_handler() {
    let pool = setup_test_db();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/api/bookmarks", web::get().to(get_all_bookmarks))
    ).await;

    let req = test::TestRequest::get()
        .uri("/api/bookmarks")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_get_bookmark_by_id_handler() {
    let pool = setup_test_db();
    let mut conn = pool.get().unwrap();
    let user = create_test_user(&mut conn);
    let category = create_test_category(&mut conn, &user);

    let new_bookmark = NewBookmark {
        title: "Test Bookmark",
        url: "https://example.com",
        description: Some("Test description"),
        category_id: category.id,
        user_id: user.id,
    };
    let bookmark = db::create_bookmark(&mut conn, new_bookmark).unwrap();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/api/bookmarks/{id}", web::get().to(get_bookmark_by_id))
    ).await;

    let req = test::TestRequest::get()
        .uri(&format!("/api/bookmarks/{}", bookmark.id))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_get_nonexistent_bookmark() {
    let pool = setup_test_db();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/api/bookmarks/{id}", web::get().to(get_bookmark_by_id))
    ).await;

    let fake_id = Uuid::new_v4();
    let req = test::TestRequest::get()
        .uri(&format!("/api/bookmarks/{}", fake_id))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 500);
}

#[actix_web::test]
async fn test_search_bookmarks_handler() {
    let pool = setup_test_db();
    let mut conn = pool.get().unwrap();
    let user = create_test_user(&mut conn);
    let category = create_test_category(&mut conn, &user);

    let new_bookmark = NewBookmark {
        title: "Searchable Bookmark",
        url: "https://searchable.com",
        description: Some("This is searchable"),
        category_id: category.id,
        user_id: user.id,
    };
    db::create_bookmark(&mut conn, new_bookmark).unwrap();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/api/bookmarks/search", web::get().to(search_bookmarks))
    ).await;

    let req = test::TestRequest::get()
        .uri("/api/bookmarks/search?q=Searchable")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_search_bookmarks_empty_query() {
    let pool = setup_test_db();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/api/bookmarks/search", web::get().to(search_bookmarks))
    ).await;

    let req = test::TestRequest::get()
        .uri("/api/bookmarks/search")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_get_bookmarks_by_category_handler() {
    let pool = setup_test_db();
    let mut conn = pool.get().unwrap();
    let user = create_test_user(&mut conn);
    let category = create_test_category(&mut conn, &user);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/api/categories/{id}/bookmarks", web::get().to(get_bookmarks_by_category))
    ).await;

    let req = test::TestRequest::get()
        .uri(&format!("/api/categories/{}/bookmarks", category.id))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}


#[actix_web::test]
async fn test_full_bookmark_lifecycle() {
    let pool = setup_test_db();
    let mut conn = pool.get().unwrap();
    let user = create_test_user(&mut conn);
    let category = create_test_category(&mut conn, &user);

    let new_bookmark = NewBookmark {
        title: "Lifecycle Test",
        url: "https://lifecycle.com",
        description: Some("Testing full lifecycle"),
        category_id: category.id,
        user_id: user.id,
    };
    let bookmark = db::create_bookmark(&mut conn, new_bookmark).unwrap();
    assert_eq!(bookmark.title, "Lifecycle Test");

    let found = db::get_bookmark_by_id(&mut conn, &bookmark.id).unwrap();
    assert_eq!(found.title, "Lifecycle Test");

    let update_data = crate::model::UpdateBookmark {
        title: Some("Updated Lifecycle"),
        url: None,
        description: None,
        category_id: None,
    };
    let updated = db::update_bookmark(&mut conn, &bookmark.id, update_data).unwrap();
    assert_eq!(updated.title, "Updated Lifecycle");

    let deleted = db::delete_bookmark(&mut conn, &bookmark.id).unwrap();
    assert_eq!(deleted, 1);

    let find_result = db::get_bookmark_by_id(&mut conn, &bookmark.id);
    assert!(find_result.is_err());
}

#[actix_web::test]
async fn test_full_user_authentication_flow() {
    let pool = setup_test_db();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/api/register", web::post().to(register))
            .route("/api/login", web::post().to(login))
    ).await;

    let username = format!("authuser_{}", Uuid::new_v4());
    let register_req = test::TestRequest::post()
        .uri("/api/register")
        .set_json(&RegisterRequest {
            username: username.clone(),
            email: format!("auth_{}@example.com", Uuid::new_v4()),
            password: "password123".to_string(),
            full_name: Some("Auth User".to_string()),
        })
        .to_request();

    let register_resp = test::call_service(&app, register_req).await;
    assert!(register_resp.status().is_success());

    let login_req = test::TestRequest::post()
        .uri("/api/login")
        .set_json(&LoginRequest {
            username: username.clone(),
            password: "password123".to_string(),
        })
        .to_request();

    let login_resp = test::call_service(&app, login_req).await;
    assert!(login_resp.status().is_success());
}

#[actix_web::test]
async fn test_full_category_workflow() {
    let pool = setup_test_db();
    let mut conn = pool.get().unwrap();
    let user = create_test_user(&mut conn);

    let new_category = NewCategory {
        name: "Workflow Category",
        user_id: user.id,
    };
    let category = db::create_category(&mut conn, new_category).unwrap();
    assert_eq!(category.name, "Workflow Category");

    let found_category = db::get_category_by_id(&mut conn, &category.id).unwrap();
    assert_eq!(found_category.id, category.id);

    let user_categories = db::get_user_categories(&mut conn, &user.id).unwrap();
    assert!(user_categories.iter().any(|c| c.id == category.id));
}

#[tokio::test] async fn test_partial_bookmark_update() {
    let pool = setup_test_db();
    let mut conn = pool.get().unwrap();
    let user = create_test_user(&mut conn);
    let category = create_test_category(&mut conn, &user);

    let new_bookmark = NewBookmark {
        title: "Partial Update Test",
        url: "https://partialupdate.com",
        description: Some("Original description"),
        category_id: category.id,
        user_id: user.id,
    };
    let bookmark = db::create_bookmark(&mut conn, new_bookmark).unwrap();

    let update_data = crate::model::UpdateBookmark {
        title: Some("New Title Only"),
        url: None,
        description: None,
        category_id: None,
    };

    let updated = db::update_bookmark(&mut conn, &bookmark.id, update_data).unwrap();
    assert_eq!(updated.title, "New Title Only");
    assert_eq!(updated.url, "https://partialupdate.com");
}

#[tokio::test] async fn test_search_case_insensitive() {
    let pool = setup_test_db();
    let mut conn = pool.get().unwrap();
    let user = create_test_user(&mut conn);
    let category = create_test_category(&mut conn, &user);

    let new_bookmark = NewBookmark {
        title: "CaSe InSeNsItIvE",
        url: "https://case.com",
        description: Some("Test case insensitivity"),
        category_id: category.id,
        user_id: user.id,
    };
    db::create_bookmark(&mut conn, new_bookmark).unwrap();

    let results = db::search_bookmarks(&mut conn, "case").unwrap();
    assert!(results.iter().any(|b| b.title.contains("CaSe")));

    let results = db::search_bookmarks(&mut conn, "INSENSITIVE").unwrap();
    assert!(results.iter().any(|b| b.title.contains("InSeNsItIvE")));
}

#[tokio::test] async fn test_multiple_users_isolation() {
    let pool = setup_test_db();
    let mut conn = pool.get().unwrap();

    let user1 = create_test_user(&mut conn);
    let user2 = create_test_user(&mut conn);

    let category1 = create_test_category(&mut conn, &user1);
    let category2 = create_test_category(&mut conn, &user2);

    let user1_categories = db::get_user_categories(&mut conn, &user1.id).unwrap();
    assert!(user1_categories.iter().any(|c| c.id == category1.id));
    assert!(!user1_categories.iter().any(|c| c.id == category2.id));

    let user2_categories = db::get_user_categories(&mut conn, &user2.id).unwrap();
    assert!(user2_categories.iter().any(|c| c.id == category2.id));
    assert!(!user2_categories.iter().any(|c| c.id == category1.id));
}

