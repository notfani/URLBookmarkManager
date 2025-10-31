#[cfg(test)]
mod db_tests {
    use diesel::prelude::*;
    use uuid::Uuid;
    use crate::tests::tests::{setup_test_db, create_test_user, create_test_category};
    use crate::model::{NewBookmark, NewCategory, NewUser};
    use crate::db;

    #[test]
    fn test_create_user_with_invalid_email() {
        let pool = setup_test_db();
        let mut conn = pool.get().unwrap();

        let password_hash = bcrypt::hash("password123", bcrypt::DEFAULT_COST).unwrap();
        let new_user = NewUser {
            username: format!("testuser_{}", Uuid::new_v4()),
            email: format!("test+special_{}@example.com", Uuid::new_v4()),
            password_hash,
            full_name: Some("Test User".to_string()),
        };

        let result = db::create_user(&mut conn, new_user);
        assert!(result.is_ok());
    }

    #[test]
    fn test_find_user_by_username_case_sensitive() {
        let pool = setup_test_db();
        let mut conn = pool.get().unwrap();

        let user = create_test_user(&mut conn);
        let uppercase_username = user.username.to_uppercase();

        let found_user = db::find_user_by_username(&mut conn, &uppercase_username);
        assert!(found_user.is_err());
    }

    #[test]
    fn test_create_category_with_long_name() {
        let pool = setup_test_db();
        let mut conn = pool.get().unwrap();
        let user = create_test_user(&mut conn);

        let long_name = "A".repeat(255);
        let new_category = NewCategory {
            name: &long_name,
            user_id: user.id,
        };

        let result = db::create_category(&mut conn, new_category);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().name, long_name);
    }

    #[test]
    fn test_create_bookmark_with_long_url() {
        let pool = setup_test_db();
        let mut conn = pool.get().unwrap();
        let user = create_test_user(&mut conn);
        let category = create_test_category(&mut conn, &user);

        let long_url = format!("https://example.com/{}", "a".repeat(500));
        let new_bookmark = NewBookmark {
            title: "Test Bookmark",
            url: &long_url,
            description: None,
            category_id: category.id,
            user_id: user.id,
        };

        let result = db::create_bookmark(&mut conn, new_bookmark);
        assert!(result.is_ok());
    }

    #[test]
    fn test_update_bookmark_with_null_description() {
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
            title: None,
            url: None,
            description: Some(""),
            category_id: None,
        };

        let result = db::update_bookmark(&mut conn, &bookmark.id, update_data);
        assert!(result.is_ok());
    }

    #[test]
    fn test_delete_nonexistent_bookmark() {
        let pool = setup_test_db();
        let mut conn = pool.get().unwrap();

        let fake_id = Uuid::new_v4();
        let result = db::delete_bookmark(&mut conn, &fake_id);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn test_search_bookmarks_with_special_characters() {
        let pool = setup_test_db();
        let mut conn = pool.get().unwrap();
        let user = create_test_user(&mut conn);
        let category = create_test_category(&mut conn, &user);

        let new_bookmark = NewBookmark {
            title: "Test: Special & Characters",
            url: "https://example.com",
            description: Some("Testing % wildcard"),
            category_id: category.id,
            user_id: user.id,
        };
        db::create_bookmark(&mut conn, new_bookmark).unwrap();

        let results = db::search_bookmarks(&mut conn, "Special").unwrap();
        assert!(results.iter().any(|b| b.title.contains("Special")));
    }

    #[test]
    fn test_get_bookmarks_by_nonexistent_category() {
        let pool = setup_test_db();
        let mut conn = pool.get().unwrap();

        let fake_id = Uuid::new_v4();
        let result = db::get_bookmarks_by_category(&mut conn, &fake_id);

        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }

    #[test]
    fn test_get_category_by_nonexistent_id() {
        let pool = setup_test_db();
        let mut conn = pool.get().unwrap();

        let fake_id = Uuid::new_v4();
        let result = db::get_category_by_id(&mut conn, &fake_id);

        assert!(result.is_err());
    }

    #[test]
    fn test_create_multiple_categories_same_name_different_users() {
        let pool = setup_test_db();
        let mut conn = pool.get().unwrap();

        let user1 = create_test_user(&mut conn);
        let user2 = create_test_user(&mut conn);

        let category_name = "Work";

        let new_category1 = NewCategory {
            name: category_name,
            user_id: user1.id,
        };
        let category1 = db::create_category(&mut conn, new_category1).unwrap();

        let new_category2 = NewCategory {
            name: category_name,
            user_id: user2.id,
        };
        let category2 = db::create_category(&mut conn, new_category2).unwrap();

        assert_eq!(category1.name, category2.name);
        assert_ne!(category1.id, category2.id);
        assert_ne!(category1.user_id, category2.user_id);
    }

    #[test]
    fn test_search_empty_string() {
        let pool = setup_test_db();
        let mut conn = pool.get().unwrap();
        let user = create_test_user(&mut conn);
        let category = create_test_category(&mut conn, &user);

        let new_bookmark = NewBookmark {
            title: "Test Bookmark",
            url: "https://example.com",
            description: Some("Test"),
            category_id: category.id,
            user_id: user.id,
        };
        db::create_bookmark(&mut conn, new_bookmark).unwrap();

        let results = db::search_bookmarks(&mut conn, "").unwrap();
        assert!(results.len() > 0);
    }

    #[test]
    fn test_bookmark_created_at_updated_at() {
        let pool = setup_test_db();
        let mut conn = pool.get().unwrap();
        let user = create_test_user(&mut conn);
        let category = create_test_category(&mut conn, &user);

        let new_bookmark = NewBookmark {
            title: "Timestamp Test",
            url: "https://example.com",
            description: None,
            category_id: category.id,
            user_id: user.id,
        };
        let bookmark = db::create_bookmark(&mut conn, new_bookmark).unwrap();
        assert!(bookmark.created_at.timestamp() > 0);
        assert!(bookmark.updated_at.timestamp() > 0);

        assert_eq!(bookmark.created_at.timestamp(), bookmark.updated_at.timestamp());
    }

    #[test]
    fn test_category_created_at_updated_at() {
        let pool = setup_test_db();
        let mut conn = pool.get().unwrap();
        let user = create_test_user(&mut conn);

        let new_category = NewCategory {
            name: "Timestamp Test",
            user_id: user.id,
        };
        let category = db::create_category(&mut conn, new_category).unwrap();

        assert!(category.created_at.timestamp() > 0);
        assert!(category.updated_at.timestamp() > 0);
    }

    #[test]
    fn test_user_created_at_updated_at() {
        let pool = setup_test_db();
        let mut conn = pool.get().unwrap();

        let password_hash = bcrypt::hash("password123", bcrypt::DEFAULT_COST).unwrap();
        let new_user = NewUser {
            username: format!("timeuser_{}", Uuid::new_v4()),
            email: format!("time_{}@example.com", Uuid::new_v4()),
            password_hash,
            full_name: Some("Time User".to_string()),
        };
        let user = db::create_user(&mut conn, new_user).unwrap();

        assert!(user.created_at.timestamp() > 0);
        assert!(user.updated_at.timestamp() > 0);
    }
}

