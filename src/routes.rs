use actix_web::web;
use crate::handlers;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/register", web::post().to(handlers::register))
            .route("/login", web::post().to(handlers::login))
    )
    .service(
        web::scope("/content")
            .route("", web::post().to(handlers::create_content))
            .route("/{id}", web::get().to(handlers::get_content))
            .route("/{id}", web::put().to(handlers::update_content))
            .route("/{id}", web::delete().to(handlers::delete_content))
    );
}