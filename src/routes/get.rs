use actix_web::HttpResponse;

use askama::Template;

use crate::templates::FormTemplate;

pub async fn get_form() -> HttpResponse {
    let template = FormTemplate {
    };

    HttpResponse::Ok()
    .content_type("text/html")
    .body(template.render().unwrap())
}
