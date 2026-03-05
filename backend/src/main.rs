use axum::{
    routing::{get, post, delete},
    Router,
    extract::{State, Path},
    Json,
    response::IntoResponse,
    middleware as ax_middleware,
};
use hyper::StatusCode;
use surrealdb::Surreal;
use surrealdb::engine::local::Mem;
use std::sync::Arc;
use shared::{ContentType, Entry};
use tower_http::{cors::CorsLayer, services::ServeDir};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use std::collections::HashMap;
use tokio::sync::RwLock;

mod auth;
mod storage;

#[derive(Clone)]
pub struct AppState {
    pub db: Surreal<surrealdb::engine::local::Db>,
    pub sessions: Arc<auth::SessionManager>,
}

#[tokio::main]
pub async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db = Surreal::new::<Mem>(()).await.expect("Failed to connect to in-memory SurrealDB");
    db.use_ns("cms").use_db("cms").await.expect("Failed to use namespace/db");

    // Initialize Auth (Scopes, initial admin)
    auth::init_auth(&db).await.expect("Failed to initialize auth");

    let sessions = Arc::new(auth::SessionManager {
        tokens: RwLock::new(HashMap::new()),
    });

    let state = Arc::new(AppState { db, sessions });

    // API routes requiring authentication
    let api_routes = Router::new()
        .route("/content-types", get(get_content_types).post(create_content_type))
        .route("/content-types/:slug", get(get_content_type).delete(delete_content_type))
        .route("/entries/:slug", get(get_entries).post(create_entry))
        .route("/entries/:slug/:id", delete(delete_entry))
        .route("/upload", post(storage::upload_image))
        .layer(ax_middleware::from_fn_with_state(state.clone(), auth::auth_middleware));

    let app = Router::new()
        .route("/api/login", post(auth::login))
        .nest("/api", api_routes)
        .nest_service("/public", ServeDir::new("public"))
        .fallback(get(move |_req: axum::extract::Request| async move {
            (
                axum::http::StatusCode::OK,
                axum::response::Html(format!(
                    "<html><body><h1>Dioxus Fullstack CMS</h1><p>SSR and Hydration are handled by the Axum fallback. In production, this would serve the hydrated <code>frontend</code> app bundle.</p><script src='/public/tailwind.css'></script></body></html>"
                ))
            ).into_response()
        }))
        .with_state(state)
        .layer(CorsLayer::permissive());

    println!("Backend server running on http://localhost:8080");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_content_types(State(state): State<Arc<AppState>>) -> Json<Vec<ContentType>> {
    let content_types: Vec<ContentType> = state.db.select("content_type").await.unwrap_or_default();
    Json(content_types)
}

async fn create_content_type(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ContentType>,
) -> impl IntoResponse {
    let created: Result<Option<ContentType>, _> = state.db.create(("content_type", payload.slug.clone())).content(payload).await;
    match created {
        Ok(ct) => (StatusCode::CREATED, Json(ct)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn get_content_type(
    State(state): State<Arc<AppState>>,
    Path(slug): Path<String>,
) -> impl IntoResponse {
    let ct: Option<ContentType> = state.db.select(("content_type", slug)).await.unwrap_or_default();
    match ct {
        Some(ct) => Json(ct).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

async fn delete_content_type(
    State(state): State<Arc<AppState>>,
    Path(slug): Path<String>,
) -> impl IntoResponse {
    let _: Option<ContentType> = state.db.delete(("content_type", slug)).await.unwrap_or_default();
    StatusCode::NO_CONTENT
}

async fn get_entries(
    State(state): State<Arc<AppState>>,
    Path(slug): Path<String>,
) -> Json<Vec<Entry>> {
    let entries: Vec<Entry> = state.db
        .query("SELECT * FROM entry WHERE content_type_slug = $slug")
        .bind(("slug", slug))
        .await
        .unwrap()
        .take(0)
        .unwrap();
    Json(entries)
}

async fn create_entry(
    State(state): State<Arc<AppState>>,
    Path(_slug): Path<String>,
    Json(payload): Json<Entry>,
) -> impl IntoResponse {
    let created: Result<Option<Entry>, _> = state.db.create("entry").content(payload).await;
    match created {
        Ok(entry) => (StatusCode::CREATED, Json(entry)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn delete_entry(
    State(state): State<Arc<AppState>>,
    Path(_slug): Path<String>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let _: Option<Entry> = state.db.delete(("entry", id)).await.unwrap_or_default();
    StatusCode::NO_CONTENT
}
