use actix_web::web;
use actix_web::App;
use actix_web::HttpServer;

mod routes;

use crate::routes::get::get_form;
use crate::routes::http_404_handler::handle_404;
use crate::routes::post::handle_xkcd_json;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .default_service(web::route().to(handle_404))
            .route("/form", web::get().to(get_form))
            .route("/xkcd", web::post().to(handle_xkcd_json))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
