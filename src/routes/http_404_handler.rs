use actix_web::HttpResponse;
use actix_web::Responder;

use askama::Template;

use crate::templates::Http404Template;

pub async fn handle_404() -> impl Responder {
    let template = Http404Template {
    };

    HttpResponse::NotFound()
    .content_type("text/html")
    .body(template.render().unwrap())
}
