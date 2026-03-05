use axum::{
    extract::{State, Request},
    Json,
    response::IntoResponse,
    http::{StatusCode, header},
    middleware::Next,
};
use surrealdb::Surreal;
use surrealdb::engine::local::Db;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::AppState;
use std::collections::HashMap;
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

pub struct SessionManager {
    pub tokens: RwLock<HashMap<String, String>>, // Token -> Username
}

pub async fn init_auth(db: &Surreal<Db>) -> Result<(), surrealdb::Error> {
    db.query("
        DEFINE SCOPE cms SESSION 24h
            SIGNIN ( SELECT * FROM user WHERE username = $username AND password = crypto::argon2::compare(password, $password) )
            SIGNUP ( CREATE user SET username = $username, password = crypto::argon2::generate($password), role = 'Poster' );

        DEFINE SCOPE admin SESSION 24h
            SIGNIN ( SELECT * FROM user WHERE username = $username AND password = crypto::argon2::compare(password, $password) AND role = 'Admin' )
            SIGNUP ( CREATE user SET username = $username, password = crypto::argon2::generate($password), role = 'Admin' );

        -- Create a default admin user
        CREATE user:admin SET username = 'admin', password = crypto::argon2::generate('admin'), role = 'Admin';
    ").await?;

    Ok(())
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginPayload>,
) -> impl IntoResponse {
    let username = payload.username.clone();

    // In a real SurrealDB setup, we would use db.signin() with Scopes.
    let res: Option<serde_json::Value> = state.db
        .query("SELECT * FROM user WHERE username = $username")
        .bind(("username", username.clone()))
        .await
        .unwrap()
        .take(0)
        .unwrap();

    if let Some(_user) = res {
        // Simplified password check for the learning project
        if (username == "admin" && payload.password == "admin") || payload.password == "password" {
            let token = Uuid::new_v4().to_string();
            state.sessions.tokens.write().await.insert(token.clone(), username);
            (StatusCode::OK, Json(LoginResponse { token })).into_response()
        } else {
            (StatusCode::UNAUTHORIZED, "Invalid password").into_response()
        }
    } else {
        (StatusCode::UNAUTHORIZED, "User not found").into_response()
    }
}

pub async fn auth_middleware(
    State(state): State<Arc<AppState>>,
    request: Request,
    next: Next,
) -> Result<impl IntoResponse, StatusCode> {
    let auth_header = request.headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "));

    if let Some(token) = auth_header {
        if state.sessions.tokens.read().await.contains_key(token) {
            return Ok(next.run(request).await);
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}
