#![allow(unused)]

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_derive_enum;

use actix_cors::Cors;
use actix_web::{App, HttpServer};
use anyhow::Result;
use listenfd::ListenFd;
use tera::Tera;

use crate::config::Config;
use crate::db::new_pool;

mod config;
mod db;
mod models;
mod queries;
mod routes;
mod schema;

#[actix_web::main]
async fn main() -> Result<()> {
    match std::fs::read("output.log") {
        Ok(_) => std::fs::remove_file("output.log").unwrap(),
        Err(_) => println!("no log file"),
    };

    match setup_logger() {
        Ok(()) => {}
        Err(err) => println!("Error: {}, while configuring logger", err),
    };

    let config = Config::new();
    let mut listenfd = ListenFd::from_env();

    let pool = new_pool(&config);

    let mut server = HttpServer::new(move || {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();

        App::new()
            .data(pool.clone())
            .data(tera)
            .wrap(Cors::permissive().max_age(3600))
            .configure(routes::endpoints)
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => server.bind(config.address)?,
    };

    server.run().await?;

    Ok(())
}

pub fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}
