pub mod controller{
    pub mod root;
}

use actix_web::web;
pub fn configure_controllers(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("").configure(self::controller::root::configure));
}