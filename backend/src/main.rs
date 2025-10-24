use actix_web::{web, App, HttpServer, middleware::Logger};
use actix_cors::Cors;
use dotenv::dotenv;

mod model;
mod schema;
mod handlers;
mod db;
mod config;
mod auth;
mod middleware;

use handlers::*;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    dotenv().ok();

    let config = config::Config::from_env().expect("Failed to load config");
    let database_url = config.database_url;
    let server_port = config.server_port;

    let pool = db::create_pool(&database_url)
        .expect("Failed to create database pool");

    println!("Starting server at http://localhost:{}", server_port);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .supports_credentials();

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(cors)
            .wrap(Logger::default())
            .route("/api/register", web::post().to(register))
            .route("/api/login", web::post().to(login))
            .route("/api/me", web::get().to(get_current_user))
            .route("/api/bookmarks", web::post().to(create_bookmark))
            .route("/api/bookmarks", web::get().to(get_all_bookmarks))
            .route("/api/bookmarks/{id}", web::get().to(get_bookmark_by_id))
            .route("/api/bookmarks/{id}", web::put().to(update_bookmark))
            .route("/api/bookmarks/{id}", web::delete().to(delete_bookmark))
            .route("/api/bookmarks/search", web::get().to(search_bookmarks))
            .route("/api/categories", web::post().to(create_category))
            .route("/api/categories", web::get().to(get_all_categories))
            .route("/api/categories/{id}/bookmarks", web::get().to(get_bookmarks_by_category))
    })
        .bind(("0.0.0.0", server_port))?
        .run()
        .await
}