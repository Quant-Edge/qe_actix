mod app;
mod handler;

use crate::app::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    run().await
}
