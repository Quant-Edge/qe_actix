mod app;
mod handler;
mod common;
mod config;

use crate::app::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    run().await
}
