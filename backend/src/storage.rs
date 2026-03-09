use axum::{
    extract::{Multipart, State},
    response::IntoResponse,
    http::StatusCode,
};
use std::sync::Arc;
use crate::AppState;
use std::fs;

pub async fn upload_image(
    State(_state): State<Arc<AppState>>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    if let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.file_name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        let path = format!("public/uploads/{}", name);
        fs::create_dir_all("public/uploads").unwrap();
        fs::write(&path, data).unwrap();

        return (StatusCode::OK, format!("/public/uploads/{}", name)).into_response();
    }

    (StatusCode::BAD_REQUEST, "No file uploaded").into_response()
}
