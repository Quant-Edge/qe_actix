use actix_web::{HttpResponse, Responder, get, post};

pub mod derivatives_trading_usds_futures;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{
        http::{self, header::ContentType},
        test,
    };

    #[actix_web::test]
    async fn test_health_check_ok() {
        let app = test::init_service(actix_web::App::new().service(health_check)).await;
        let req = test::TestRequest::default()
            .insert_header(ContentType::plaintext())
            .uri("/health_check")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_health_check_not_ok() {
        let app = test::init_service(actix_web::App::new().service(health_check)).await;
        let req = test::TestRequest::default()
            .uri("/health_check")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success()); // 修改断言，因为没有内容类型头部也应该成功
    }
}
