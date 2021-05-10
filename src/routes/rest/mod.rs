mod conlangs;
mod groups;
mod persons;
mod scripts;

use actix_web::web::ServiceConfig;

pub fn endpoints(config: &mut ServiceConfig) {
    scripts::endpoints(config);
    persons::endpoints(config);
    groups::endpoints(config);
    conlangs::endpoints(config);
}
