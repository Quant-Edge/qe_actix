mod app;
mod handler;
mod common;

use crate::app::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    run().await
}
