use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use actix_web::web;

use serde::Deserialize;

use askama::Template;

use crate::templates::Http404XkcdTemplate;
use crate::templates::XkcdImageTemplate;

#[derive(Deserialize)]
pub struct XkcdId {
    pub id: u16,
}

#[derive(Deserialize)]
pub struct XkcdResp {
    pub img: String,
}

pub async fn handle_xkcd_json(form: web::Form<XkcdId>) -> Result<HttpResponse, actix_web::Error> {
    let xkcd_com_resp = awc::Client::new()
                        .get(format!("https://xkcd.com/{}/info.0.json", form.id))
                        .send()
                        .await;

    match xkcd_com_resp {
        Ok(mut response) => {
            match response.status() {
                StatusCode::OK => {
                    let xkcd_resp = response.json::<XkcdResp>()
                                    .await
                                    .unwrap();

                    let xkcd_img_template = XkcdImageTemplate {
                        img_url: &xkcd_resp.img
                    };

                    return Ok(HttpResponse::Ok()
                              .content_type("text/html")
                              .body(xkcd_img_template.render().unwrap()))
                }
                StatusCode::NOT_FOUND => {
                    let xkcd_404_template = Http404XkcdTemplate {
                        id: form.id
                    };

                    return Ok(HttpResponse::NotFound()
                              .content_type("text/html")
                              .body(xkcd_404_template.render().unwrap()))
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
