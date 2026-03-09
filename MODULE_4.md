# Module 4: Axum Server Configuration

A Dioxus Fullstack server is a standard Axum server that's specifically configured to serve the Dioxus app.

## Key Concepts
1. **AppState**: A thread-safe, shared container (wrapped in `Arc`) that holds our SurrealDB connection.
2. **Routes**: We use Axum's router to define `/api` endpoints for our CMS operations.
3. **Middleware**: We use `tower-http`'s `CorsLayer` for handling cross-origin requests.
4. **ServeDir**: In a production environment, we use `tower_http::services::ServeDir` to serve the static WASM bundle and asset files (images, CSS).

## SurrealDB Integration
In our backend:
1. We initialize an **In-Memory SurrealDB** (`Surreal::new::<Mem>(())`).
2. We define **Scopes** for Authentication (`SIGNIN` and `SIGNUP` blocks).
3. We perform standard CRUD operations using SurrealDB's `select`, `create`, and `query` methods.

## Serving the Fullstack App
Our Axum server is configured with the `dioxus-fullstack` server handler. This handler intercepts requests to specific routes and performs the SSR logic before sending the final HTML.
