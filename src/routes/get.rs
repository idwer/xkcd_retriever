use actix_web::HttpResponse;
use actix_web::Responder;

use askama::Template;

use crate::templates::FormTemplate;

pub async fn get_form() -> impl Responder {
    let template = FormTemplate {
    };

    HttpResponse::Ok()
    .content_type("text/html")
    .body(template.render().unwrap())
}
