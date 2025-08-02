use actix_web::{HttpResponse, post, web};
use binance_sdk::derivatives_trading_usds_futures::rest_api::{
    self, ChangeMarginTypeMarginTypeEnum, ChangeMarginTypeParams,
};
use serde::Deserialize;
use tracing::error;

use crate::{app::AppState, common::params::KeyName, handler::common::get_client_from_state};

#[derive(Deserialize)]
struct ChangeMarginTypeParamsWrapper {
    symbol: String,
    margin_type: ChangeMarginTypeMarginTypeEnum,
}

impl From<ChangeMarginTypeParamsWrapper> for ChangeMarginTypeParams {
    fn from(wrapper: ChangeMarginTypeParamsWrapper) -> Self {
        let builder = Self::builder(wrapper.symbol, wrapper.margin_type);
        builder.build().unwrap()
    }
}

#[post("/change_margin_type")]
async fn change_margin_type(
    data: web::Data<AppState>,
    query: web::Query<KeyName>,
    param: web::Form<ChangeMarginTypeParamsWrapper>,
) -> Result<HttpResponse, actix_web::Error> {
    // 调用辅助函数获取客户端
    let client = get_client_from_state::<rest_api::RestApi>(&data, &query.key)?;

    // 设置 API 参数
    let params =
        rest_api::ChangeMarginTypeParams::builder(param.symbol.clone(), param.margin_type.clone())
            .build()
            .unwrap();

    // 调用 API 方法，替换为实际存在的 get_account_info 方法
    let response = client.change_margin_type(params).await.map_err(|e| {
        error!("change_margin_type: {}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    let data = response.data().await.map_err(|e| {
        error!("Failed to get data from response: {}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    // 返回响应
    Ok(HttpResponse::Ok().json(data))
}
