use actix_web::{HttpResponse, post, web};
use binance_sdk::{
    derivatives_trading_usds_futures::rest_api::{self, NewOrderParams, NewOrderSideEnum},
    spot::rest_api::NewOrderTypeEnum,
};
use rust_decimal::Decimal;
use serde::Deserialize;
use tracing::error;

use crate::{app::AppState, common::params::KeyName, handler::common::get_client_from_state};

#[derive(Deserialize)]
struct NewOrderParamsWrapper {
    symbol: String,
    side: NewOrderSideEnum,
    r#type: NewOrderTypeEnum,
    quantity: Decimal,
    price: Option<Decimal>,
    // time_in_force: Option<String>,
}

impl From<NewOrderParamsWrapper> for NewOrderParams {
    fn from(wrapper: NewOrderParamsWrapper) -> Self {
        let mut builder = Self::builder(
            wrapper.symbol,
            wrapper.side,
            wrapper.r#type.as_str().to_string(),
        )
        .quantity(wrapper.quantity);

        if let Some(price) = wrapper.price {
            builder = builder.price(price);
        }

        // if let Some(time_in_force) = wrapper.time_in_force {
        //     builder = builder.time_in_force(time_in_force);
        // }

        builder.build().unwrap()
    }
}

/// 创建新订单
/// POST /new_order
/// 参数:
/// - symbol: 交易对 (必填)
/// - side: 订单方向 (必填)
/// - type: 订单类型 (必填)
/// - quantity: 数量 (必填)
/// - price: 价格 (限价单必填)
/// - time_in_force: 有效时间 (可选)
#[post("/new_order")]
async fn new_order(
    data: web::Data<AppState>,
    query: web::Query<KeyName>,
    param: web::Form<NewOrderParamsWrapper>,
) -> Result<HttpResponse, actix_web::Error> {
    // 调用辅助函数获取客户端
    let client = get_client_from_state::<rest_api::RestApi>(&data, &query.key)?;

    // 调用 API 方法
    let response = client
        .new_order(param.into_inner().into())
        .await
        .map_err(|e| {
            error!("new_order: {}", e);
            actix_web::error::ErrorInternalServerError(e)
        })?;

    let data = response.data().await.map_err(|e| {
        error!("Failed to get data from response: {}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    // 返回响应
    Ok(HttpResponse::Ok().json(data))
}
