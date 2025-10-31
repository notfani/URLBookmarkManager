#[cfg(test)]
mod handler_tests {
    use actix_web::{test, web, App, http::header};
    use uuid::Uuid;
    use crate::tests::tests::{setup_test_db, create_test_user, create_test_category};
    use crate::model::{RegisterRequest, LoginRequest, NewBookmark};
    use crate::handlers::*;
    use crate::db;
    use crate::auth;

    #[actix_web::test]
    async fn test_delete_bookmark_handler() {
        let pool = setup_test_db();
        let mut conn = pool.get().unwrap();
        let user = create_test_user(&mut conn);
        let category = create_test_category(&mut conn, &user);

        let new_bookmark = NewBookmark {
            title: "Delete Test",
            url: "https://delete.com",
            description: None,
            category_id: category.id,
            user_id: user.id,
        };
        let bookmark = db::create_bookmark(&mut conn, new_bookmark).unwrap();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .route("/api/bookmarks/{id}", web::delete().to(delete_bookmark))
        ).await;

        let req = test::TestRequest::delete()
            .uri(&format!("/api/bookmarks/{}", bookmark.id))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_update_bookmark_handler() {
        let pool = setup_test_db();
        let mut conn = pool.get().unwrap();
        let user = create_test_user(&mut conn);
        let category = create_test_category(&mut conn, &user);

        let new_bookmark = NewBookmark {
            title: "Original",
            url: "https://original.com",
            description: Some("Original description"),
            category_id: category.id,
            user_id: user.id,
        };
        let bookmark = db::create_bookmark(&mut conn, new_bookmark).unwrap();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .route("/api/bookmarks/{id}", web::put().to(update_bookmark))
        ).await;

        let req = test::TestRequest::put()
            .uri(&format!("/api/bookmarks/{}", bookmark.id))
            .set_json(&UpdateBookmarkRequest {
                title: Some("Updated Title".to_string()),
                url: None,
                description: None,
                category_id: None,
            })
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_register_with_minimal_data() {
        let pool = setup_test_db();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .route("/api/register", web::post().to(register))
        ).await;

        let req = test::TestRequest::post()
            .uri("/api/register")
            .set_json(&RegisterRequest {
                username: format!("minuser_{}", Uuid::new_v4()),
                email: format!("min_{}@example.com", Uuid::new_v4()),
                password: "password123".to_string(),
                full_name: None,
            })
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_register_with_short_password() {
        let pool = setup_test_db();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .route("/api/register", web::post().to(register))
        ).await;

        let req = test::TestRequest::post()
            .uri("/api/register")
            .set_json(&RegisterRequest {
                username: format!("shortpw_{}", Uuid::new_v4()),
                email: format!("short_{}@example.com", Uuid::new_v4()),
                password: "123".to_string(),
                full_name: Some("Short Password User".to_string()),
            })
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_login_with_wrong_username() {
        let pool = setup_test_db();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .route("/api/login", web::post().to(login))
        ).await;

        let req = test::TestRequest::post()
            .uri("/api/login")
            .set_json(&LoginRequest {
                username: "nonexistent_user_12345".to_string(),
                password: "password123".to_string(),
            })
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status().as_u16(), 401);
    }

    #[actix_web::test]
    async fn test_search_with_no_results() {
        let pool = setup_test_db();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .route("/api/bookmarks/search", web::get().to(search_bookmarks))
        ).await;

        let req = test::TestRequest::get()
            .uri("/api/bookmarks/search?q=nonexistentquery12345xyz")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_get_bookmarks_empty_category() {
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
    async fn test_api_response_structure() {
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

        let content_type = resp.headers().get(header::CONTENT_TYPE);
        assert!(content_type.is_some());
        assert!(content_type.unwrap().to_str().unwrap().contains("application/json"));
    }

    #[actix_web::test]
    async fn test_login_response_contains_token() {
        let pool = setup_test_db();
        let mut conn = pool.get().unwrap();

        let password_hash = bcrypt::hash("password123", bcrypt::DEFAULT_COST).unwrap();
        let new_user = crate::model::NewUser {
            username: format!("tokenuser_{}", Uuid::new_v4()),
            email: format!("token_{}@example.com", Uuid::new_v4()),
            password_hash,
            full_name: Some("Token User".to_string()),
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

        let body = test::read_body(resp).await;
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        assert!(body_str.contains("token"));
    }

    #[actix_web::test]
    async fn test_register_response_contains_token() {
        let pool = setup_test_db();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .route("/api/register", web::post().to(register))
        ).await;

        let req = test::TestRequest::post()
            .uri("/api/register")
            .set_json(&RegisterRequest {
                username: format!("regtoken_{}", Uuid::new_v4()),
                email: format!("regtoken_{}@example.com", Uuid::new_v4()),
                password: "password123".to_string(),
                full_name: Some("Register Token User".to_string()),
            })
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let body = test::read_body(resp).await;
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        assert!(body_str.contains("token"));
    }

    #[actix_web::test]
    async fn test_multiple_concurrent_requests() {
        let pool = setup_test_db();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .route("/api/bookmarks", web::get().to(get_all_bookmarks))
        ).await;

        let req1 = test::TestRequest::get().uri("/api/bookmarks").to_request();
        let req2 = test::TestRequest::get().uri("/api/bookmarks").to_request();
        let req3 = test::TestRequest::get().uri("/api/bookmarks").to_request();

        let resp1 = test::call_service(&app, req1).await;
        let resp2 = test::call_service(&app, req2).await;
        let resp3 = test::call_service(&app, req3).await;

        assert!(resp1.status().is_success());
        assert!(resp2.status().is_success());
        assert!(resp3.status().is_success());
    }
}

