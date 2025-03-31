use actix_web::{web, App, HttpServer, middleware, HttpResponse};
use sqlx::postgres::PgPoolOptions;

mod routes;
mod models;
mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .service(web::scope("/api")
                .configure(routes::config))
            .default_service(
                web::route().to(|| HttpResponse::NotFound().json("404 Not Found"))
            )
    })
    .bind(("0.0.0.0", 5555))?
    .run()
    .await
}