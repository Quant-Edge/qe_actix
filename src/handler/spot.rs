pub mod get;
pub mod post;

use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/spot")
            .service(post::kline),
    );
}
