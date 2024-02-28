use std::sync::Arc;

use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use utoipa_swagger_ui::SwaggerUi;

use configs::common::ApplicationConfig;
use databases::async_postgres::AsyncPostgresPool;
use helper::logger::initialize_logger;
use routes::user_routes::UserRoutes;

use crate::helper::swagger_docs::ApiDoc;
use crate::middlewares::timer_middleware::{TimerMiddleware, TimerMiddlewareTransform};
use crate::routes::auth_routes::AuthRoutes;
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
    let configs = Arc::new(ApplicationConfig::new()); // Creates arc reference to share across threads
    let state = Arc::clone(&configs); // This reference is used to pass the application state to
                                      // the routes

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
            .app_data(web::Data::new(state.clone()))
            .app_data(web::Data::new(pool.pool.clone()))
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .uri("/api-docs/openapi.json", ApiDoc::openapi()),
            )
            .route(
                "/health",
                web::to(|| async { HttpResponse::Ok().json("OK") }),
            )
            .route("/users", web::post().to(UserRoutes::create))
            .route("/users/{id}", web::get().to(UserRoutes::get))
            .route("/users/{id}", web::patch().to(UserRoutes::update))
            .route("/users/{id}", web::delete().to(UserRoutes::delete))
            .route("/schools", web::post().to(SchoolRoutes::create))
            .route("/schools/{id}", web::get().to(SchoolRoutes::get))
            .route("/schools/{id}", web::patch().to(SchoolRoutes::update))
            .route("/schools/{id}", web::delete().to(SchoolRoutes::delete))
            .route("/auth/login", web::post().to(AuthRoutes::login))
            .route(
                "/auth/authenticate",
                web::get().to(AuthRoutes::authenticate),
            )
    })
    .bind(format!(
        "{}:{}",
        Arc::clone(&configs).server.app_host,
        Arc::clone(&configs).server.app_port //&configs.server.app_host, &configs.server.app_port
    ))?
    .run()
    .await
}
