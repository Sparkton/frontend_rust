use shared::{ContentType, Entry};
use reqwest::Client;
use serde_json::Value;

static AUTH_TOKEN: std::sync::OnceLock<String> = std::sync::OnceLock::new();

pub fn set_token(token: String) {
    let _ = AUTH_TOKEN.set(token);
}

fn get_token() -> Option<&'static String> {
    AUTH_TOKEN.get()
}

pub async fn login(username: String, password: String) -> Result<String, String> {
    let client = Client::new();
    let res = client.post("http://localhost:8080/api/login")
        .json(&serde_json::json!({ "username": username, "password": password }))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if res.status().is_success() {
        let body: Value = res.json().await.map_err(|e| e.to_string())?;
        let token = body["token"].as_str().unwrap().to_string();
        set_token(token.clone());
        Ok(token)
    } else {
        Err("Login failed".into())
    }
}

pub async fn fetch_content_types() -> Vec<ContentType> {
    let client = Client::new();
    let mut req = client.get("http://localhost:8080/api/content-types");
    if let Some(token) = get_token() {
        req = req.bearer_auth(token);
    }

    req.send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap_or_default()
}

pub async fn create_content_type(ct: ContentType) {
    let client = Client::new();
    let mut req = client.post("http://localhost:8080/api/content-types");
    if let Some(token) = get_token() {
        req = req.bearer_auth(token);
    }

    req.json(&ct)
        .send()
        .await
        .unwrap();
}

pub async fn fetch_entries(slug: String) -> Vec<Entry> {
    let client = Client::new();
    let mut req = client.get(format!("http://localhost:8080/api/entries/{}", slug));
    if let Some(token) = get_token() {
        req = req.bearer_auth(token);
    }

    req.send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap_or_default()
}
