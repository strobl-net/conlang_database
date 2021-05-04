use actix_web::web::ServiceConfig;

mod rest;

pub fn endpoints(config: &mut ServiceConfig) {
    rest::endpoints(config);
}
