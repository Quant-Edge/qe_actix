use actix_web::HttpResponse;
use actix_web::{get, web};
use binance_sdk::spot::rest_api::{ExchangeInfoParams, ExchangeInfoSymbolStatusEnum};
use tracing::error;

use crate::{app::AppState, common::params::KeyName};

use crate::handler::common::get_client_from_state;

#[get("/exchange_information")]
async fn exchange_information(
    data: web::Data<AppState>,
    query: web::Query<KeyName>,
) -> Result<HttpResponse, actix_web::Error> {
    /*
    https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Kline-Candlestick-Data
    kline candlestick data 和 continuous contract kline candlestick data 传参不一样, 获取的数据是一样的
    之所以有这样的区别, 论坛上给出的答复是为了给使用 stream 的人方便调用的, 我们这里使用 kline candlestick data
    */

    // 调用辅助函数获取客户端
    let client = get_client_from_state::<binance_sdk::spot::rest_api::RestApi>(&data, &query.key)?;

    // Setup the API parameters
    let params = ExchangeInfoParams::builder()
        .permissions(vec!["SPOT".to_string()])
        .show_permission_sets(false)
        .symbol_status(ExchangeInfoSymbolStatusEnum::Trading)
        .build()
        .unwrap();

    // 调用 API 方法，替换为实际存在的 get_account_info 方法
    let response = client.exchange_info(params).await.map_err(|e| {
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
