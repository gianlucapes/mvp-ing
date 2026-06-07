pub mod plate2d;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(plate2d::create_plate2d);
}