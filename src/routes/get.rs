use actix_web::{HttpResponse, Responder};

pub async fn get_form() -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body(
        r#"<html><body>
            <form action="/xkcd" method="POST">
            <p>XKCD ID: <input name = "id" type = "number" value=200 /></p>
            <p><input type = "submit" value = "Submit" /></p>
            </form>
            </html></body>"#,
    )
}
