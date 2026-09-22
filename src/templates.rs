use askama::Template;

#[derive(Template)]
#[template(path = "form.html")]
pub struct FormTemplate {
}

#[derive(Template)]
#[template(path = "404.html")]
pub struct Http404Template {
}

#[derive(Template)]
#[template(path = "404_xkcd.html")]
pub struct Http404XkcdTemplate {
    pub id: u16
}

#[derive(Template)]
#[template(path = "xkcd_image.html")]
pub struct XkcdImageTemplate<'a> {
    pub img_url: &'a str
}
