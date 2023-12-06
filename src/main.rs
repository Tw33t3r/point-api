mod model;
mod route;
mod schema;

use std::env;
use actix_web::{web, App, HttpServer};
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};

pub struct AppState {
    db: SqlitePool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let args: Vec<String> = env::args().collect();

    let db_path = &args[1];
    //TODO: add Postgres option
    let pool = match SqlitePoolOptions::new()
        .max_connections(10)
        .connect(db_path)
        .await
    {
        Ok(pool) => {
            println!("Connected to database");
            pool
        }
        Err(err) => {
            println!("Failed to connect to database: {:?}", err);
            std::process::exit(1);
        }
    };
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .service(route::get_user_balance)
            .service(route::post_add_points)
            .service(route::post_sub_points)
            .service(route::post_new_order)
    })
    .bind(("localhost", 4000))?
    .run()
    .await
}
