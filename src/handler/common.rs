use actix_web::web;
use binance_sdk::derivatives_trading_usds_futures::rest_api::RestApi;
use tracing::error;

use crate::app::AppState;

pub fn get_client_from_state(
    data: &web::Data<AppState>,
    key_name: &str,
) -> Result<RestApi, actix_web::Error> {
    // 直接从应用状态获取 rest_clients
    let rest_clients = data.rest_usds_future_clients.clone();

    // 使用 let 绑定延长锁的生命周期
    let locked_clients = rest_clients.lock().unwrap();

    // 从 rest_clients 中获取指定的客户端
    locked_clients
        .get(key_name)
        .ok_or_else(|| {
            error!("Failed to get client for key_name: {}", key_name);
            actix_web::error::ErrorBadRequest(format!(
                "Client not found for key_name: {}",
                key_name
            ))
        })
        .cloned()
}
