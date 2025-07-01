use actix_web::{App, HttpServer, web};

use crate::handler::{index, health_check, echo, manual_hello};


pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(health_check)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}