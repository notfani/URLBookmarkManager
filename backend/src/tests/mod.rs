#[cfg(test)]
mod tests {
    use actix_web::{test, web, App};
    use diesel::prelude::*;
    use diesel::r2d2::{self, ConnectionManager};
    use uuid::Uuid;
    use crate::models::{Bookmark, NewBookmark, Category, NewCategory};
    use crate::db;
    use crate::handlers::*;

    type Pool = r2d2::Pool<ConnectionManager<diesel::PgConnection>>;

    fn setup_test_db() -> Pool {
        let database_url = std::env::var("TEST_DATABASE_URL")
            .expect("TEST_DATABASE_URL must be set for tests");

        let manager = ConnectionManager::<diesel::PgConnection>::new(&database_url);
        r2d2::Pool::builder()
            .max_size(1)
            .build(manager)
            .expect("Failed to create test pool")
    }

    #[test]
    fn test_create_bookmark() {
        let pool = setup_test_db();
        let mut conn = pool.get().unwrap();

        let new_category = NewCategory {
            name: "Test Category",
        };

        let category = db::create_category(&mut conn, new_category).unwrap();

        let new_bookmark = NewBookmark {
            title: "Test Bookmark",
            url: "https://example.com",
            description: Some("Test description"),
            category_id: category.id,
        };

        let result = db::create_bookmark(&mut conn, new_bookmark);

        assert!(result.is_ok());
        let bookmark = result.unwrap();
        assert_eq!(bookmark.title, "Test Bookmark");
        assert_eq!(bookmark.url, "https://example.com");
    }

    #[test]
    fn test_search_bookmarks() {
        let pool = setup_test_db();
        let mut conn = pool.get().unwrap();

        let new_category = NewCategory {
            name: "Test Category",
        };

        let category = db::create_category(&mut conn, new_category).unwrap();

        let bookmark1 = NewBookmark {
            title: "Rust Programming",
            url: "https://www.rust-lang.org",
            description: Some("Official Rust website"),
            category_id: category.id,
        };

        let bookmark2 = NewBookmark {
            title: "Google",
            url: "https://www.google.com",
            description: Some("Search engine"),
            category_id: category.id,
        };

        db::create_bookmark(&mut conn, bookmark1).unwrap();
        db::create_bookmark(&mut conn, bookmark2).unwrap();

        let results = db::search_bookmarks(&mut conn, "Rust").unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, "Rust Programming");

        let results = db::search_bookmarks(&mut conn, "google").unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, "Google");
    }

    #[actix_web::test]
    async fn test_create_bookmark_handler() {
        let pool = setup_test_db();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .route("/api/bookmarks", web::post().to(create_bookmark))
        ).await;

        let mut conn = pool.get().unwrap();
        let new_category = NewCategory {
            name: "Test Category",
        };
        let category = db::create_category(&mut conn, new_category).unwrap();

        let req = test::TestRequest::post()
            .uri("/api/bookmarks")
            .set_json(&CreateBookmarkRequest {
                title: "Test Bookmark".to_string(),
                url: "https://example.com".to_string(),
                description: Some("Test description".to_string()),
                category_id: category.id,
            })
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
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
}