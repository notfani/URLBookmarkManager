#[cfg(test)]
mod auth_tests {
    use uuid::Uuid;
    use crate::auth;
    use actix_web::{test, HttpRequest};
    use chrono::{Duration, Utc};

    #[test]
    fn test_jwt_token_format() {
        let user_id = Uuid::new_v4();
        let username = "testuser";

        let token = auth::create_jwt(&user_id, username).unwrap();
        let parts: Vec<&str> = token.split('.').collect();
        assert_eq!(parts.len(), 3);
    }

    #[test]
    fn test_jwt_claims_match() {
        let user_id = Uuid::new_v4();
        let username = "testuser123";

        let token = auth::create_jwt(&user_id, username).unwrap();
        let claims = auth::verify_jwt(&token).unwrap();

        assert_eq!(claims.username, username);
        assert_eq!(claims.sub, user_id.to_string());
    }

    #[test]
    fn test_jwt_expiration_is_set() {
        let user_id = Uuid::new_v4();
        let username = "exptest";

        let token = auth::create_jwt(&user_id, username).unwrap();
        let claims = auth::verify_jwt(&token).unwrap();

        let now = Utc::now().timestamp();
        assert!(claims.exp > now);

        let expected_exp = Utc::now()
            .checked_add_signed(Duration::hours(24))
            .unwrap()
            .timestamp();

        assert!((claims.exp - expected_exp).abs() < 5);
    }

    #[test]
    fn test_jwt_with_empty_username() {
        let user_id = Uuid::new_v4();
        let username = "";

        let token = auth::create_jwt(&user_id, username).unwrap();
        let claims = auth::verify_jwt(&token).unwrap();

        assert_eq!(claims.username, username);
    }

    #[test]
    fn test_jwt_with_special_characters_in_username() {
        let user_id = Uuid::new_v4();
        let username = "user@#$%^&*()";

        let token = auth::create_jwt(&user_id, username).unwrap();
        let claims = auth::verify_jwt(&token).unwrap();

        assert_eq!(claims.username, username);
    }

    #[test]
    fn test_jwt_with_unicode_username() {
        let user_id = Uuid::new_v4();
        let username = "用户名";

        let token = auth::create_jwt(&user_id, username).unwrap();
        let claims = auth::verify_jwt(&token).unwrap();

        assert_eq!(claims.username, username);
    }

    #[test]
    fn test_verify_jwt_with_malformed_token() {
        let result = auth::verify_jwt("notajwt");
        assert!(result.is_err());
    }

    #[test]
    fn test_verify_jwt_with_empty_string() {
        let result = auth::verify_jwt("");
        assert!(result.is_err());
    }

    #[test]
    fn test_verify_jwt_with_wrong_signature() {
        let user_id = Uuid::new_v4();
        let username = "testuser";

        let mut token = auth::create_jwt(&user_id, username).unwrap();

        token.push_str("corrupted");

        let result = auth::verify_jwt(&token);
        assert!(result.is_err());
    }

    #[test]
    fn test_jwt_token_uniqueness() {
        let user_id1 = Uuid::new_v4();
        let user_id2 = Uuid::new_v4();
        let username = "sameusername";

        let token1 = auth::create_jwt(&user_id1, username).unwrap();
        let token2 = auth::create_jwt(&user_id2, username).unwrap();

        assert_ne!(token1, token2);
    }

    #[test]
    fn test_jwt_consistency() {
        let user_id = Uuid::new_v4();
        let username = "consistencytest";
        let token1 = auth::create_jwt(&user_id, username).unwrap();

        std::thread::sleep(std::time::Duration::from_millis(10));

        let token2 = auth::create_jwt(&user_id, username).unwrap();

        assert_ne!(token1, token2);

        let claims1 = auth::verify_jwt(&token1).unwrap();
        let claims2 = auth::verify_jwt(&token2).unwrap();

        assert_eq!(claims1.sub, claims2.sub);
        assert_eq!(claims1.username, claims2.username);
    }

    #[test]
    fn test_jwt_with_max_length_username() {
        let user_id = Uuid::new_v4();
        let username = "a".repeat(255);

        let token = auth::create_jwt(&user_id, &username).unwrap();
        let claims = auth::verify_jwt(&token).unwrap();

        assert_eq!(claims.username, username);
    }

    #[test]
    fn test_multiple_jwt_verifications() {
        let user_id = Uuid::new_v4();
        let username = "multitest";

        let token = auth::create_jwt(&user_id, username).unwrap();

        for _ in 0..10 {
            let result = auth::verify_jwt(&token);
            assert!(result.is_ok());

            let claims = result.unwrap();
            assert_eq!(claims.username, username);
        }
    }

    #[test]
    fn test_jwt_parsing_different_uuids() {
        let test_cases = vec![
            Uuid::new_v4(),
            Uuid::nil(),
            Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap(),
        ];

        for user_id in test_cases {
            let token = auth::create_jwt(&user_id, "testuser").unwrap();
            let claims = auth::verify_jwt(&token).unwrap();

            assert_eq!(claims.sub, user_id.to_string());
        }
    }

    #[test]
    fn test_jwt_token_is_base64_encoded() {
        let user_id = Uuid::new_v4();
        let username = "b64test";

        let token = auth::create_jwt(&user_id, username).unwrap();

        let parts: Vec<&str> = token.split('.').collect();
        for part in parts {
            assert!(part.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_'));
        }
    }
}

