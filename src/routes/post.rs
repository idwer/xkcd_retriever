use actix_web::{HttpResponse, web};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct XkcdId {
    pub id: u16,
}

#[derive(Deserialize)]
pub struct XkcdResp {
    pub img: String,
}

pub async fn handle_xkcd_json(data: web::Form<XkcdId>) -> HttpResponse {
    let xcd_resp = awc::Client::new()
        .get(format!("https://xkcd.com/{}/info.0.json", data.id))
        .send()
        .await
        .unwrap()
        .json::<XkcdResp>()
        .await
        .unwrap();

    HttpResponse::Ok().content_type("text/html").body(format!(
        r#"<html><body><img src={}></body></html>"#,
        xcd_resp.img
    ))
}
