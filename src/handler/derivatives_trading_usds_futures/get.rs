use actix_web::{HttpResponse, get, web, web::Query};
use binance_sdk::derivatives_trading_usds_futures::rest_api;
use tracing::error;

use crate::app::AppState;
use crate::common::params::KeyName;

use crate::handler::common::get_client_from_state;


#[get("/account_information")]
async fn account_information(
    data: web::Data<AppState>,
    query: Query<KeyName>,
) -> Result<HttpResponse, actix_web::Error> {
    // 调用辅助函数获取客户端
    let client = get_client_from_state(&data, &query.key)?;

    // 设置 API 参数
    let params = rest_api::AccountInformationV3Params::default();

    // 调用 API 方法，替换为实际存在的 get_account_info 方法
    let response = client.account_information_v3(params).await.map_err(|e| {
        error!("Failed to get account information: {}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    let data = response.data().await.map_err(|e| {
        error!("Failed to get data from response: {}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    // 返回响应
    Ok(HttpResponse::Ok().json(data))
}

#[get("/futures_account_balance")]
async fn futures_account_balance(
    data: web::Data<AppState>,
    query: Query<KeyName>,
) -> Result<HttpResponse, actix_web::Error> {
    // 调用辅助函数获取客户端
    let client = get_client_from_state(&data, &query.key)?;

    // 设置 API 参数
    let params = rest_api::FuturesAccountBalanceV3Params::default();

    // 调用 API 方法，替换为实际存在的 get_account_info 方法
    let response = client
        .futures_account_balance_v3(params)
        .await
        .map_err(|e| {
            error!("futures_account_balance_v3: {}", e);
            actix_web::error::ErrorInternalServerError(e)
        })?;

    let data = response.data().await.map_err(|e| {
        error!("Failed to get data from response: {}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    // 返回响应
    Ok(HttpResponse::Ok().json(data))
}