use actix_web::{HttpResponse, get, web, web::Query};
use binance_sdk::derivatives_trading_usds_futures::rest_api::AccountInformationV3Params;
use tracing::info;

use crate::app::AppState;

#[derive(serde::Deserialize)]
struct KeyNameParams {
    key: String,
}

#[get("/account_information")]
async fn account_information(
    data: web::Data<AppState>,
    query: Query<KeyNameParams>,
) -> Result<HttpResponse, actix_web::Error> {
    // 移除获取凭证和构建 REST 配置的代码
    // 直接从应用状态获取 rest_clients
    let rest_clients = data.rest_clients.clone();

    // 从请求中获取 key_name
    let key_name = query.key.clone();

    // 使用 let 绑定延长锁的生命周期
    let locked_clients = rest_clients.lock().unwrap();

    // 从 rest_clients 中获取指定的客户端
    let client = locked_clients.get(&key_name).ok_or_else(|| {
        info!("Failed to get client for key_name: {}", key_name);
        actix_web::error::ErrorBadRequest(format!("Client not found for key_name: {}", key_name))
    })?;

    // 设置 API 参数
    let params = AccountInformationV3Params::default();

    // 调用 API 方法，替换为实际存在的 get_account_info 方法
    let response = client.account_information_v3(params).await.map_err(|e| {
        info!("Failed to get account information: {}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    let data = response.data().await.map_err(|e| {
        info!("Failed to get data from response: {}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    // 返回响应
    Ok(HttpResponse::Ok().json(data))
}

/*
use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::derivatives_trading_usds_futures::{
    DerivativesTradingUsdsFuturesRestApi, rest_api::AccountInformationV3Params,
};

#[tokio::main]
async fn main() -> Result<()> {
    // Load credentials from env
    let api_key = env::var("API_KEY").context("API_KEY must be set")?;
    let api_secret = env::var("API_SECRET").context("API_SECRET must be set")?;

    // Build REST config
    let rest_conf = ConfigurationRestApi::builder()
        .api_key(api_key)
        .api_secret(api_secret)
        .build()?;

    // Create the DerivativesTradingUsdsFutures REST API client
    let rest_client = DerivativesTradingUsdsFuturesRestApi::production(rest_conf);

    // Setup the API parameters
    let params = AccountInformationV3Params::default();

    // Make the API call
    let response = rest_client
        .account_information_v3(params)
        .await
        .context("account_information_v3 request failed")?;

    info!(?response.rate_limits, "account_information_v3 rate limits");
    let data = response.data().await?;
    info!(?data, "account_information_v3 data");

    Ok(())
}
*/
