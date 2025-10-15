use actix_web::http::StatusCode;
use actix_web::HttpResponse;

pub async fn handle_404() -> HttpResponse {
    HttpResponse::build(StatusCode::NOT_FOUND)
           .content_type("text/html; charset=utf-8")
           .body(r#"<html><body><img src="https://img.freepik.com/free-vector/oops-404-error-with-broken-robot-concept-illustration_114360-5529.jpg"></html></body>"#.to_string())
}
