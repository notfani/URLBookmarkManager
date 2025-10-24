// @generated automatically by Diesel CLI.

diesel::table! {
    bookmarks (id) {
        id -> Uuid,
        title -> Varchar,
        url -> Varchar,
        description -> Nullable<Varchar>,
        category_id -> Uuid,
        user_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    categories (id) {
        id -> Uuid,
        name -> Varchar,
        user_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        username -> Varchar,
        email -> Varchar,
        password_hash -> Varchar,
        full_name -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(bookmarks -> categories (category_id));
diesel::joinable!(bookmarks -> users (user_id));
diesel::joinable!(categories -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    bookmarks,
    categories,
    users,
);