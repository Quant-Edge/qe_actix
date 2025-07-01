use actix_web::{HttpResponse, get, web, web::Query};
use binance_sdk::derivatives_trading_usds_futures::rest_api;
use tracing::error;

use crate::app::AppState;
use crate::common::params::KeyName;

#[get("/account_information")]
async fn account_information(
    data: web::Data<AppState>,
    query: Query<KeyName>,
) -> Result<HttpResponse, actix_web::Error> {
    // 直接从应用状态获取 rest_clients
    let rest_clients = data.rest_clients.clone();

    // 从请求中获取 key_name
    let key_name = query.key.clone();

    // 使用 let 绑定延长锁的生命周期
    let locked_clients = rest_clients.lock().unwrap();

    // 从 rest_clients 中获取指定的客户端
    let client = locked_clients.get(&key_name).ok_or_else(|| {
        error!("Failed to get client for key_name: {}", key_name);
        actix_web::error::ErrorBadRequest(format!("Client not found for key_name: {}", key_name))
    })?;

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
    // 直接从应用状态获取 rest_clients
    let rest_clients = data.rest_clients.clone();

    // 从请求中获取 key_name
    let key_name = query.key.clone();

    // 使用 let 绑定延长锁的生命周期
    let locked_clients = rest_clients.lock().unwrap();

    // 从 rest_clients 中获取指定的客户端
    let client = locked_clients.get(&key_name).ok_or_else(|| {
        error!("Failed to get client for key_name: {}", key_name);
        actix_web::error::ErrorBadRequest(format!("Client not found for key_name: {}", key_name))
    })?;

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
