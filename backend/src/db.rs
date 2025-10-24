use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use diesel::prelude::*;
use crate::model::{Bookmark, NewBookmark, UpdateBookmark, Category, NewCategory, BookmarkWithCategory, User, NewUser};
use uuid::Uuid;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn create_pool(database_url: &str) -> Result<Pool, r2d2::PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder().build(manager)
}

pub fn establish_connection(database_url: &str) -> PgConnection {
    PgConnection::establish(database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_bookmark(
    conn: &mut PgConnection,
    new_bookmark: NewBookmark,
) -> Result<Bookmark, diesel::result::Error> {
    use crate::schema::bookmarks;

    diesel::insert_into(bookmarks::table)
        .values(&new_bookmark)
        .get_result(conn)
}

pub fn get_all_bookmarks(
    conn: &mut PgConnection,
) -> Result<Vec<BookmarkWithCategory>, diesel::result::Error> {
    use crate::schema::bookmarks::dsl::*;
    use crate::schema::categories;

    bookmarks
        .inner_join(categories::table)
        .select((
            crate::schema::bookmarks::all_columns,
            categories::all_columns,
        ))
        .load::<(Bookmark, Category)>(conn)
        .map(|results| {
            results.into_iter()
                .map(|(bookmark, category)| bookmark.with_category(category))
                .collect()
        })
}

pub fn get_bookmark_by_id(
    conn: &mut PgConnection,
    bookmark_id: &Uuid,
) -> Result<BookmarkWithCategory, diesel::result::Error> {
    use crate::schema::bookmarks::dsl::*;
    use crate::schema::categories;

    bookmarks
        .filter(id.eq(bookmark_id))
        .inner_join(categories::table)
        .select((
            crate::schema::bookmarks::all_columns,
            categories::all_columns,
        ))
        .first::<(Bookmark, Category)>(conn)
        .map(|(bookmark, category)| bookmark.with_category(category))
}

pub fn update_bookmark(
    conn: &mut PgConnection,
    bookmark_id: &Uuid,
    update_data: UpdateBookmark,
) -> Result<Bookmark, diesel::result::Error> {
    use crate::schema::bookmarks::dsl::*;

    diesel::update(bookmarks.filter(id.eq(bookmark_id)))
        .set(&update_data)
        .get_result(conn)
}

pub fn delete_bookmark(
    conn: &mut PgConnection,
    bookmark_id: &Uuid,
) -> Result<usize, diesel::result::Error> {
    use crate::schema::bookmarks::dsl::*;

    diesel::delete(bookmarks.filter(id.eq(bookmark_id))).execute(conn)
}

pub fn search_bookmarks(
    conn: &mut PgConnection,
    query: &str,
) -> Result<Vec<BookmarkWithCategory>, diesel::result::Error> {
    use crate::schema::bookmarks::dsl::*;
    use crate::schema::categories;

    bookmarks
        .filter(title.ilike(format!("%{}%", query)))
        .inner_join(categories::table)
        .select((
            crate::schema::bookmarks::all_columns,
            categories::all_columns,
        ))
        .load::<(Bookmark, Category)>(conn)
        .map(|results| {
            results.into_iter()
                .map(|(bookmark, category)| bookmark.with_category(category))
                .collect()
        })
}

pub fn get_bookmarks_by_category(
    conn: &mut PgConnection,
    cat_id: &Uuid,
) -> Result<Vec<BookmarkWithCategory>, diesel::result::Error> {
    use crate::schema::bookmarks::dsl::*;
    use crate::schema::categories;

    bookmarks
        .filter(category_id.eq(cat_id))
        .inner_join(categories::table)
        .select((
            crate::schema::bookmarks::all_columns,
            categories::all_columns,
        ))
        .load::<(Bookmark, Category)>(conn)
        .map(|results| {
            results.into_iter()
                .map(|(bookmark, category)| bookmark.with_category(category))
                .collect()
        })
}

pub fn create_category(
    conn: &mut PgConnection,
    new_category: NewCategory,
) -> Result<Category, diesel::result::Error> {
    use crate::schema::categories;

    diesel::insert_into(categories::table)
        .values(&new_category)
        .get_result(conn)
}

pub fn get_all_categories(
    conn: &mut PgConnection,
) -> Result<Vec<Category>, diesel::result::Error> {
    use crate::schema::categories::dsl::*;

    categories.load::<Category>(conn)
}

pub fn get_category_by_id(
    conn: &mut PgConnection,
    category_id: &Uuid,
) -> Result<Category, diesel::result::Error> {
    use crate::schema::categories::dsl::*;

    categories.filter(id.eq(category_id)).first(conn)
}

pub fn get_user_categories(
    conn: &mut PgConnection,
    uid: &Uuid,
) -> Result<Vec<Category>, diesel::result::Error> {
    use crate::schema::categories::dsl::*;

    categories.filter(user_id.eq(uid)).load::<Category>(conn)
}

pub fn create_user(
    conn: &mut PgConnection,
    new_user: NewUser,
) -> Result<User, diesel::result::Error> {
    use crate::schema::users;

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
}

pub fn find_user_by_username(
    conn: &mut PgConnection,
    user_name: &str,
) -> Result<User, diesel::result::Error> {
    use crate::schema::users::dsl::*;

    users.filter(username.eq(user_name)).first(conn)
}

pub fn find_user_by_email(
    conn: &mut PgConnection,
    user_email: &str,
) -> Result<User, diesel::result::Error> {
    use crate::schema::users::dsl::*;

    users.filter(email.eq(user_email)).first(conn)
}

pub fn get_user_by_id(
    conn: &mut PgConnection,
    user_id: &Uuid,
) -> Result<User, diesel::result::Error> {
    use crate::schema::users::dsl::*;

    users.filter(id.eq(user_id)).first(conn)
}
