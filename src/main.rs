use actix_cors::Cors;
use actix_web::{App, HttpServer};
use anyhow::Result;
use listenfd::ListenFd;
use sqlx::PgPool;
use tera::Tera;

use crate::config::Config;

mod config;
mod models;
mod routes;
mod db;

#[actix_web::main]
async fn main() -> Result<()> {
    let config = Config::new();
    let mut listenfd = ListenFd::from_env();

    let pool = PgPool::connect(&config.db_address).await?;

    let mut server = HttpServer::new(move || {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();

        App::new()
            .data(pool.clone())
            .data(tera)
            .wrap(Cors::permissive().max_age(3600))
            // .configure(routes::endpoints)
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => server.bind(config.address)?,
    };

    server.run().await?;

    Ok(())
}
