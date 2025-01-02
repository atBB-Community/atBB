use axum::{extract::State, Json};

use crate::models::forum::CategoryForum;

pub async fn get_forums(State(forums): State<Vec<CategoryForum>>) -> Json<Vec<CategoryForum>> {
    Json(forums)
}
