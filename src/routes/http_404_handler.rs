use actix_web::HttpResponse;

use askama::Template;

use crate::templates::Http404Template;

pub async fn handle_404() -> HttpResponse {
    let template = Http404Template {
    };

    HttpResponse::NotFound()
    .content_type("text/html")
    .body(template.render().unwrap())
}
