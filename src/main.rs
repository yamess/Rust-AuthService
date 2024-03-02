use actix_web::{middleware, web, App, HttpResponse, HttpServer};

use configs::common::ApplicationConfig;
use databases::async_postgres::AsyncPostgresPool;
use helper::logger::initialize_logger;
use routes::user_routes::UserRoutes;

use crate::routes::auth_routes::AuthRoutes;
use crate::routes::class_routes::ClassRoutes;
use crate::routes::password_routes::PasswordRoutes;
use crate::routes::schedule_routes::ScheduleRoutes;
use crate::routes::school_routes::SchoolRoutes;
use crate::routes::student_routes::StudentRoutes;

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
            .service(
                web::scope("/users")
                    .route("", web::post().to(UserRoutes::create))
                    .route("/{id}", web::get().to(UserRoutes::get))
                    .route("/{id}", web::put().to(UserRoutes::update))
                    .route("/{id}", web::delete().to(UserRoutes::delete))
                    .route("/{id}/password", web::put().to(PasswordRoutes::update)),
            )
            .service(
                web::scope("/schools")
                    .route("", web::post().to(SchoolRoutes::create))
                    .route("/{id}", web::get().to(SchoolRoutes::get))
                    .route("/{id}", web::put().to(SchoolRoutes::update))
                    .route("/{id}", web::delete().to(SchoolRoutes::delete)),
            )
            .service(
                web::scope("/students")
                    .route("", web::post().to(StudentRoutes::create))
                    .route("/{id}", web::get().to(StudentRoutes::get))
                    .route("/{id}", web::patch().to(StudentRoutes::update))
                    .route("/{id}", web::delete().to(StudentRoutes::delete)),
            )
            .service(
                web::scope("/classes")
                    .route("", web::post().to(ClassRoutes::create))
                    .route("/{id}", web::get().to(ClassRoutes::get))
                    .route("/{id}", web::patch().to(ClassRoutes::update))
                    .route("/{id}", web::delete().to(ClassRoutes::delete)),
            )
            .service(
                web::scope("/schedules")
                    .route("", web::post().to(ScheduleRoutes::create))
                    .route("/{id}", web::get().to(ScheduleRoutes::get))
                    .route("/{id}", web::patch().to(ScheduleRoutes::update))
                    .route("/{id}", web::delete().to(ScheduleRoutes::delete)),
            )
    })
    .bind(format!(
        "{}:{}",
        &configs.server.app_host, &configs.server.app_port
    ))?
    .workers(num_cpus::get() * 2)
    .run()
    .await
}
