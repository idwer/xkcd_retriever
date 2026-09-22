use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use actix_web::web;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct XkcdId {
    pub id: u16,
}

#[derive(Deserialize)]
pub struct XkcdResp {
    pub img: String,
}

pub async fn handle_xkcd_json(data: web::Form<XkcdId>) -> Result<HttpResponse, actix_web::Error> {
    let xkcd_com_resp = awc::Client::new()
                        .get(format!("https://xkcd.com/{}/info.0.json", data.id))
                        .send()
                        .await;

    match xkcd_com_resp {
        Ok(mut response) => {
            match response.status() {
                StatusCode::OK => {
                    let xkcd_resp = response.json::<XkcdResp>()
                                    .await
                                    .unwrap();
                    return Ok(HttpResponse::Ok()
                           .content_type("text/html")
                           .body(format!(r#"<html><body><img src={}></body></html>"#, xkcd_resp.img)))
                }
                StatusCode::NOT_FOUND => {
                    return Ok(HttpResponse::Ok()
                           .content_type("text/html")
                           .body(format!(r#"<html><body>XKCD {} not found<br><img src="https://img.freepik.com/free-vector/oops-404-error-with-broken-robot-concept-illustration_114360-5529.jpg"></html></body>"#, data.id)))
                }
                _ => return Err(actix_web::error::ErrorInternalServerError(response.status()))
            }
        }

        Err(response_err) => {
            eprintln!("Error retrieving XKCD JSON: {}", response_err);

            return Err(actix_web::error::ErrorInternalServerError(response_err))
        }
    }
}
