use actix_web::{HttpResponse, get, web};
use binance_sdk::derivatives_trading_usds_futures::rest_api;
use tracing::error;

use crate::app::AppState;
use crate::common::params::KeyName;

use crate::handler::common::get_client_from_state;

#[get("/exchange_information")]
pub async fn exchange_information(
    data: web::Data<AppState>,
    query: web::Query<KeyName>,
) -> Result<HttpResponse, actix_web::Error> {
    // 调用辅助函数获取客户端
    let client = get_client_from_state::<rest_api::RestApi>(&data, &query.key)?;

    // 调用 API 方法，替换为实际存在的 get_account_info 方法
    let response = client.exchange_information().await.map_err(|e| {
        error!("exchange_information: {}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    let data = response.data().await.map_err(|e| {
        error!("Failed to get data from response: {}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    // 返回响应
    Ok(HttpResponse::Ok().json(data))
}
