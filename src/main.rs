use std::sync::Arc;

use actix_web::{middleware, web, App, HttpResponse, HttpServer};

use configs::common::ApplicationConfig;
use databases::async_postgres::AsyncPostgresPool;
use helper::logger::initialize_logger;
use routes::user_routes::UserRoutes;

mod configs;
mod databases;
mod helper;
mod interfaces;
mod models;
mod repositories;
mod routes;
mod schema;
mod schemas;
mod services;
mod tables;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configs = ApplicationConfig::new();
    let pool = AsyncPostgresPool::new(&configs.database).await;

    initialize_logger(&configs.logger.log_folder)
        .await
        .expect("Failed to initialize logger");

    log::info!("Logger initialized");
    log::info!(
        "Starting server at http://{}:{} ...",
        &configs.server.app_host,
        &configs.server.app_port
    );

    let config_to_move = Arc::clone(&configs);
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::Logger::new("%a %{User-Agent}i"))
            .wrap(middleware::Compress::default())
            .wrap(middleware::NormalizePath::new(
                middleware::TrailingSlash::Trim,
            ))
            .app_data(web::Data::new(Arc::clone(&config_to_move)))
            .app_data(web::Data::new(pool.pool.clone()))
            .route(
                "/health",
                web::to(|| async { HttpResponse::Ok().json("OK") }),
            )
            .route("/users", web::post().to(UserRoutes::create))
            .route("/users/{id}", web::get().to(UserRoutes::get))
            .route("/users/{id}", web::patch().to(UserRoutes::update))
            .route("/users/{id}", web::delete().to(UserRoutes::delete))
    })
    .bind(format!(
        "{}:{}",
        &configs.server.app_host, &configs.server.app_port
    ))?
    .run()
    .await
}
