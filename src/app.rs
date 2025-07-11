use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use actix_web::{App, HttpServer, web};
use config::{Config, File};
use serde::Deserialize;
use tracing::info;
use tracing_subscriber::fmt;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::derivatives_trading_usds_futures::DerivativesTradingUsdsFuturesRestApi;
use binance_sdk::derivatives_trading_usds_futures::rest_api::RestApi;

use crate::config::{AppConfig, load_config};
use crate::handler::usds_future;
use crate::handler::{echo, health_check, index};

#[derive(Debug, Clone, Deserialize)]
pub struct Key {
    #[serde(rename = "apiKey", alias = "api_key")]
    pub api_key: String,
    pub secret: String,
}

// 新增全局的 REST API 客户端类型定义
pub struct AppState {
    pub rest_usds_future_clients: Arc<Mutex<HashMap<String, RestApi>>>,
}

fn load_keys() -> Result<HashMap<String, Key>, config::ConfigError> {
    let keys = Config::builder()
        .add_source(File::with_name("keys.toml"))
        .build()?;

    keys.try_deserialize()
}

pub fn init_rest_usds_future_clients(
    keys: &HashMap<String, Key>,
    app_config: &AppConfig,
) -> Result<Arc<Mutex<HashMap<String, RestApi>>>, std::io::Error> {
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

        // 创建 USDS 期货交易 REST API 客户端
        let client = DerivativesTradingUsdsFuturesRestApi::production(rest_conf);
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

    let rest_clients = init_rest_usds_future_clients(&keys, &config)?;

    info!(
        "Starting server at {}:{}",
        config.server.host, config.server.port
    );

    HttpServer::new(move || {
        App::new()
            // 将 keys 和 rest_clients 一起添加到应用状态
            .app_data(web::Data::new(AppState {
                rest_usds_future_clients: rest_clients.clone(),
            }))
            .service(index)
            .service(health_check)
            .service(echo)
            .configure(usds_future::routes)
    })
    .bind((config.server.host, config.server.port))?
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
