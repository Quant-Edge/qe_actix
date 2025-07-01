use actix_web::{App, HttpServer, web};
use config::{Config, File};
use serde::Deserialize;
use tracing::info;
use tracing_subscriber::fmt;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::derivatives_trading_usds_futures::DerivativesTradingUsdsFuturesRestApi;
use binance_sdk::derivatives_trading_usds_futures::rest_api::RestApi;

use crate::handler::derivatives_trading_usds_futures::get::account_information;
use crate::handler::{echo, health_check, index, manual_hello};

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Deserialize)]
struct ServerConfig {
    host: String,
    port: u16,
}

#[derive(Debug, Deserialize)]
struct AppConfig {
    server: ServerConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Key {
    #[serde(rename = "apiKey", alias = "api_key")]
    pub api_key: String,
    pub secret: String,
}

// 新增全局的 REST API 客户端类型定义
pub struct AppState {
    pub rest_clients: Arc<Mutex<HashMap<String, RestApi>>>,
}

fn load_config() -> Result<AppConfig, config::ConfigError> {
    let config = Config::builder()
        .add_source(File::with_name("config.toml"))
        .build()?;

    config.try_deserialize()
}

fn load_keys() -> Result<HashMap<String, Key>, config::ConfigError> {
    let keys = Config::builder()
        .add_source(File::with_name("keys.toml"))
        .build()?;

    keys.try_deserialize()
}

pub fn init_rest_clients(
    keys: &HashMap<String, Key>,
) -> Result<Arc<Mutex<HashMap<String, RestApi>>>, std::io::Error> {
    // 初始化一个 HashMap 来存储每个 key 对应的 rest_client
    let mut rest_clients = HashMap::new();

    // 遍历每个 key，为其创建对应的 rest_client
    for (key_name, key) in keys.iter() {
        // 构建 REST 配置
        let rest_conf = match ConfigurationRestApi::builder()
            .api_key(key.api_key.clone())
            .api_secret(key.secret.clone())
            .build()
        {
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

    let rest_clients = init_rest_clients(&keys)?;

    info!(
        "Starting server at {}:{}",
        config.server.host, config.server.port
    );

    HttpServer::new(move || {
        App::new()
            // 将 keys 和 rest_clients 一起添加到应用状态
            .app_data(web::Data::new(AppState {
                rest_clients: rest_clients.clone(),
            }))
            .service(index)
            .service(health_check)
            .service(echo)
            .service(account_information)
            .route("/hey", web::get().to(manual_hello))
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