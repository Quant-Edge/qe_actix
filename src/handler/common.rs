use crate::app::AppState;
use actix_web::web;
use tracing::error;

// 定义特征，用于标识不同的客户端类型
pub trait ClientSelector: Clone {
    fn get_clients(
        data: &web::Data<AppState>,
    ) -> std::sync::Arc<std::sync::Mutex<std::collections::HashMap<String, Self>>>;
}

// 为不同的客户端类型实现 ClientSelector 特征
impl ClientSelector for binance_sdk::derivatives_trading_usds_futures::rest_api::RestApi {
    fn get_clients(
        data: &web::Data<AppState>,
    ) -> std::sync::Arc<std::sync::Mutex<std::collections::HashMap<String, Self>>> {
        data.rest_usds_future_clients.clone()
    }
}

impl ClientSelector for binance_sdk::spot::rest_api::RestApi {
    fn get_clients(
        data: &web::Data<AppState>,
    ) -> std::sync::Arc<std::sync::Mutex<std::collections::HashMap<String, Self>>> {
        data.rest_spot_clients.clone()
    }
}

// 使用泛型重写 get_client_from_state 函数
pub fn get_client_from_state<T: ClientSelector + Clone>(
    data: &web::Data<AppState>,
    key_name: &str,
) -> Result<T, actix_web::Error> {
    let rest_clients = T::get_clients(data);
    let locked_clients = rest_clients.lock().unwrap();
    locked_clients
        .get(key_name)
        .cloned()
        .ok_or_else(|| {
            error!("Failed to get client for key_name: {}", key_name);
            actix_web::error::ErrorBadRequest(format!(
                "Client not found for key_name: {}",
                key_name
            ))
        })
}