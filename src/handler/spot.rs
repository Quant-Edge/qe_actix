pub mod get;
pub mod post;

use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/spot")
            // GET method
            .service(get::exchange_information)
            // POST method
            .service(post::kline),
    );
}
