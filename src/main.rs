use actix_web::{middleware, web, App, HttpResponse, HttpServer};

use configs::common::ApplicationConfig;
use databases::async_postgres::AsyncPostgresPool;
use helper::logger::initialize_logger;
use routes::user_routes::UserRoutes;

use crate::routes::auth_routes::AuthRoutes;
use crate::routes::password_routes::PasswordRoutes;
use crate::routes::school_routes::SchoolRoutes;

mod configs;
mod databases;
mod helper;
mod interfaces;
mod middlewares;
mod models;
mod repositories;
mod routes;
mod schema;
mod schemas;
mod services;
mod tables;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(ApplicationConfig::new());
    let configs = state.clone();
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

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::Logger::new("%a %{User-Agent}i"))
            .wrap(middleware::Compress::default())
            .wrap(middleware::NormalizePath::new(
                middleware::TrailingSlash::Trim,
            ))
            .app_data(state.clone())
            .app_data(web::Data::new(pool.pool.clone()))
            .route(
                "/health",
                web::to(|| async { HttpResponse::Ok().json("OK") }),
            )
            .route("/auth/login", web::post().to(AuthRoutes::login))
            .route("/users", web::post().to(UserRoutes::create))
            .route("/users/{id}", web::get().to(UserRoutes::get))
            .route("/users/{id}", web::patch().to(UserRoutes::update))
            .route("/users/{id}", web::delete().to(UserRoutes::delete))
            .route(
                "/users/{id}/password",
                web::patch().to(PasswordRoutes::update),
            )
            .route("/schools", web::post().to(SchoolRoutes::create))
            .route("/schools/{id}", web::get().to(SchoolRoutes::get))
            .route("/schools/{id}", web::patch().to(SchoolRoutes::update))
            .route("/schools/{id}", web::delete().to(SchoolRoutes::delete))
    })
    .bind(format!(
        "{}:{}",
        &configs.server.app_host,
        &configs.server.app_port //&configs.server.app_host, &configs.server.app_port
    ))?
    .workers(num_cpus::get() * 2)
    .run()
    .await
}
