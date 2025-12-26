use askama::Template;
use askama_web::WebTemplate;

#[derive(Template, WebTemplate)]
#[template(path = "chat.html")]
pub struct HelloTemplate {
    name: String,
}

pub async fn hello() -> HelloTemplate {
    HelloTemplate {
        name: "User".to_string(),
    }
}
