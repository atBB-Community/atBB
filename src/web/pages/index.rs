use axum::{extract::State, response::Html};

use crate::models::forum::CategoryForum;

use super::TEMPLATES;

pub async fn index(State(forums): State<Vec<CategoryForum>>) -> Html<String> {
    let mut ctx = tera::Context::new();
    ctx.insert("forums", &forums);

    let rendered = TEMPLATES.render("index.html", &ctx).unwrap();

    Html(rendered)
}
