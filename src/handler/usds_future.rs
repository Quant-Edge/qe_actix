pub mod get;
pub mod post;

use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/usds_future")
            // GET method
            .service(get::account::account_information)
            .service(get::account::account_balance)
            .service(get::exchange::exchange_information)
            .service(get::position::position_information)
            // POST method
            .service(post::position::change_position_mode)
            .service(post::leverage::change_initial_leverage)
            .service(post::kline::kline)
            .service(post::order::new_order),
    );
}