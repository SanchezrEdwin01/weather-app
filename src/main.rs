use actix_rt;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;
use tera::Tera;
use std::io;
use yew_router::prelude::*;

mod controllers;
mod dao;
mod database;
mod models;
mod routes;
mod services;
mod views;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set in .env file");
    let pool = database::establish_connection();

    let tera = Tera::new("templates/**/*").expect("Failed to initialize Tera");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .data(tera.clone())
            .configure(routes::config)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
