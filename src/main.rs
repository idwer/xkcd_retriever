use actix_web::App;
use actix_web::HttpServer;
use actix_web::web;

mod routes;

use crate::routes::get::get_form;
use crate::routes::http_404_handler::handle_404;
use crate::routes::post::handle_xkcd_json;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let actix_routes = HttpServer::new(|| {
                App::new()
                .default_service(web::route().to(handle_404))
                .route("/xkcd", web::get().to(get_form))
                .route("/xkcd", web::post().to(handle_xkcd_json))
    })
    .bind(("127.0.0.1", 8080));

    match actix_routes {
        Ok(actix_server) => {
            actix_server.run().await
        }
        Err(actix_err) => {
            eprintln!("Logging actix_web error: {}", actix_err);

            return Err(actix_err)
        }
    }
}
