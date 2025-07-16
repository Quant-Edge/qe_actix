pub mod get;
pub mod post;

use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/usds_future")
            .service(get::account_information)
            .service(get::account_balance)
            .service(post::position::change_position_mode)
            .service(post::kline::kline)
            .service(post::order::new_order),
    );
}