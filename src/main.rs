use std::env;
use actix_web::{App, HttpServer, Responder, web};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use controller::settler_controller::{create_proposal, get_proposal};
use diesel_migrations::{embed_migrations};


mod controller;
mod converter;
mod dto;
mod model;
mod repository;
mod schema;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

async fn greet() -> impl Responder {
    "Hello, world!"
}

/// Configures routes for the web application.
fn configure_app(app: &mut web::ServiceConfig) {
    app.route("/", web::get().to(greet))
        .service(create_proposal)
        .service(get_proposal);
}

/// Starts the HTTP server with the specified application configuration.
async fn start_server(db_pool: DbPool) -> std::io::Result<()> {
    let server_address =
        env::var("SERVER_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8080".to_string());

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .configure(configure_app)
    })
    .bind(&server_address)?
    .run()
    .await
}

embed_migrations!();

fn create_db_pool() -> DbPool {
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://test:test@localhost:5432/mydb".to_string());
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = create_db_pool();
    start_server(pool).await
}
