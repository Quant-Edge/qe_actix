use actix_web::{HttpResponse, post, web, web::Query};
use binance_sdk::derivatives_trading_usds_futures::rest_api::{
    self, KlineCandlestickDataIntervalEnum, KlineCandlestickDataParams
};
use serde::Deserialize;
use tracing::error;

use crate::app::AppState;
use crate::common::params::KeyName;
use crate::handler::common::get_client_from_state;

#[derive(Deserialize)]
struct PositionModeParam {
    mode: String,
}

#[post("/change_position_mode")]
async fn change_position_mode(
    data: web::Data<AppState>,
    query: Query<KeyName>,
    param: web::Form<PositionModeParam>,
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

// 定义包装结构体
#[derive(Deserialize)]
struct KlineCandlestickDataParamsWrapper {
    symbol: String,
    interval: KlineCandlestickDataIntervalEnum,
    start_time: Option<i64>,
    end_time: Option<i64>,
    limit: Option<i64>,
}

impl From<KlineCandlestickDataParamsWrapper> for KlineCandlestickDataParams {
    fn from(wrapper: KlineCandlestickDataParamsWrapper) -> Self {
        KlineCandlestickDataParams::builder(wrapper.symbol, wrapper.interval)
            .start_time(wrapper.start_time)
            .end_time(wrapper.end_time)
            .limit(wrapper.limit)
            .build()
            .unwrap()
    }
}

#[post("/kline")]
async fn kline(
    data: web::Data<AppState>,
    query: Query<KeyName>,
    param: web::Form<KlineCandlestickDataParamsWrapper>,
) -> Result<HttpResponse, actix_web::Error> {
    /*
    https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Kline-Candlestick-Data
    kline candlestick data 和 continuous contract kline candlestick data 传参不一样, 获取的数据是一样的
    之所以有这样的区别, 论坛上给出的答复是为了给使用 stream 的人方便调用的, 我们这里使用 kline candlestick data
    */

    // 调用辅助函数获取客户端
    let client = get_client_from_state(&data, &query.key)?;

    // 调用 API 方法，替换为实际存在的 get_account_info 方法
    let response = client
        .kline_candlestick_data(param.into_inner().into())
        .await
        .map_err(|e| {
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
