use actix_web::{HttpResponse, get, web, web::Query};
use binance_sdk::derivatives_trading_usds_futures::rest_api;
use serde::Deserialize;
use tracing::error;

use crate::app::AppState;
use crate::common::params::KeyName;
use crate::handler::common::get_client_from_state;

#[derive(Deserialize)]
struct Param {
    mode: String,
}

#[get("/change_position_mode/{mode}")]
async fn change_position_mode(
    data: web::Data<AppState>,
    query: Query<KeyName>,
    param: web::Path<Param>,
) -> Result<HttpResponse, actix_web::Error> {
    // 调用辅助函数获取客户端
    let client = get_client_from_state(&data, &query.key)?;

    // 设置 API 参数
    let params = rest_api::ChangePositionModeParams::builder(param.mode.clone())
        .build()
        .unwrap();

    // 调用 API 方法，替换为实际存在的 get_account_info 方法
    let response = client.change_position_mode(params).await.map_err(|e| {
        error!("change_position_mode: {}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    let data = response.data().await.map_err(|e| {
        error!("Failed to get data from response: {}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    // 返回响应
    Ok(HttpResponse::Ok().json(data))
}
