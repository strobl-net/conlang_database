use crate::config::Config;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;

pub mod conlang;
pub mod group;
pub mod person;
pub mod script;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn new_pool(config: &Config) -> PgPool {
    let manager = ConnectionManager::<PgConnection>::new(&config.db_address);
    Pool::builder()
        .build(manager)
        .expect("Unable to build pool")
}
