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
            if response.status().is_success() {
                let xkcd_resp = response.json::<XkcdResp>()
                                .await
                                .unwrap();

                Ok(HttpResponse::Ok()
                   .content_type("text/html")
                   .body(format!(r#"<html><body><img src={}></body></html>"#, xkcd_resp.img)))
            } else {
                Err(actix_web::error::ErrorInternalServerError("http 500"))
            }
        }
        _ => {
            Err(actix_web::error::ErrorInternalServerError("http 500"))
        }
    }
}
