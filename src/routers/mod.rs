mod validate;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    validate::validate_routes(cfg);
}
