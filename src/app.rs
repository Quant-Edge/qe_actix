use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use actix_web::{App, HttpServer, web};
use config::{Config, File};
use serde::Deserialize;
use tracing::info;
use tracing_subscriber::fmt;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::derivatives_trading_usds_futures;
use binance_sdk::derivatives_trading_usds_futures::DerivativesTradingUsdsFuturesRestApi;
use binance_sdk::spot;
use binance_sdk::spot::SpotRestApi;

use crate::config::{AppConfig, load_config};
use crate::handler::usds_future as usds_future_handler;
use crate::handler::spot as sport_handler;
use crate::handler::{echo, health_check, index};

#[derive(Debug, Clone, Deserialize)]
pub struct Key {
    #[serde(rename = "apiKey", alias = "api_key")]
    pub api_key: String,
    pub secret: String,
}

// 新增全局的 REST API 客户端类型定义
pub struct AppState {
    pub rest_usds_future_clients: Arc<Mutex<HashMap<String, derivatives_trading_usds_futures::rest_api::RestApi>>>,
    pub rest_spot_clients: Arc<Mutex<HashMap<String, spot::rest_api::RestApi>>>,
}

fn load_keys() -> Result<HashMap<String, Key>, config::ConfigError> {
    let keys = Config::builder()
        .add_source(File::with_name("keys.toml"))
        .build()?;

    keys.try_deserialize()
}

// 定义 ClientBuilder 特征
pub trait ClientBuilder {
    type ApiClient; // 关联类型
    fn build(conf: ConfigurationRestApi) -> Self::ApiClient;
}

// 为 USDS 期货客户端实现 ClientBuilder 特征
impl ClientBuilder for DerivativesTradingUsdsFuturesRestApi {
    type ApiClient = derivatives_trading_usds_futures::rest_api::RestApi;
    fn build(conf: ConfigurationRestApi) -> Self::ApiClient {
        Self::production(conf)
    }
}

// 为现货客户端实现 ClientBuilder 特征
impl ClientBuilder for SpotRestApi {
    type ApiClient = spot::rest_api::RestApi;
    fn build(conf: ConfigurationRestApi) -> Self::ApiClient {
        Self::production(conf)
    }
}

// 使用泛型实现 init_rest_clients 函数
pub fn init_rest_clients<T: ClientBuilder + 'static>(
    keys: &HashMap<String, Key>,
    app_config: &AppConfig,
) -> Result<Arc<Mutex<HashMap<String, T::ApiClient>>>, std::io::Error> {
    // 初始化一个 HashMap 来存储每个 key 对应的 rest_client
    let mut rest_clients = HashMap::new();

    // 提前处理代理配置
    let proxy_config = app_config
        .proxy
        .as_ref()
        .map(|proxy| binance_sdk::config::ProxyConfig {
            host: proxy.host.clone(),
            port: proxy.port,
            protocol: proxy.protocol.clone(),
            auth: proxy
                .auth
                .as_ref()
                .map(|auth| binance_sdk::config::ProxyAuth {
                    username: auth.username.clone(),
                    password: auth.password.clone(),
                }),
        });

    // 遍历每个 key，为其创建对应的 rest_client
    for (key_name, key) in keys.iter() {
        // 构建 REST 配置
        let mut builder = ConfigurationRestApi::builder()
            .api_key(key.api_key.clone())
            .api_secret(key.secret.clone());

        // 设置代理配置
        if let Some(proxy) = &proxy_config {
            builder = builder.proxy(proxy.clone());
        }

        let rest_conf = match builder.build() {
            Ok(conf) => conf,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Failed to build REST configuration",
                ));
            }
        };

        // 根据 ClientBuilder 特征创建客户端
        // 根据 ClientBuilder 特征创建客户端
        let client: T::ApiClient = T::build(rest_conf);
        rest_clients.insert(key_name.clone(), client);
    }

    Ok(Arc::new(Mutex::new(rest_clients)))
}

pub async fn run() -> std::io::Result<()> {
    // 初始化 tracing 日志收集器
    fmt().with_max_level(tracing::Level::INFO).init();

    let config = load_config().map_err(|e| {
        std::io::Error::new(std::io::ErrorKind::Other, format!("Config error: {}", e))
    })?;

    let keys = load_keys().map_err(|e| {
        std::io::Error::new(std::io::ErrorKind::Other, format!("Keys error: {}", e))
    })?;

    // 初始化 USDS 期货客户端
    let rest_usds_future_clients =
        init_rest_clients::<DerivativesTradingUsdsFuturesRestApi>(&keys, &config)?;
    // 初始化现货客户端
    let rest_spot_clients = init_rest_clients::<SpotRestApi>(&keys, &config)?;

    info!(
        "Starting server at {}:{}",
        config.server.host, config.server.port
    );

    HttpServer::new(move || {
        App::new()
            // 修改 AppState 字段类型以匹配泛型返回类型
            .app_data(web::Data::new(AppState {
                rest_usds_future_clients: rest_usds_future_clients.clone(),
                rest_spot_clients: rest_spot_clients.clone(),
            }))
            .service(index)
            .service(health_check)
            .service(echo)
            .configure(usds_future_handler::routes)
            .configure(sport_handler::routes)
    })
    // .bind((config.server.host, config.server.port))?
    .bind(("::", config.server.port))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_keys() {
        let result = load_keys();
        assert!(result.is_ok(), "Failed to load keys: {:?}", result.err());

        let keys = result.unwrap();
        assert!(!keys.is_empty(), "No keys loaded");
    }
}