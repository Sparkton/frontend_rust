# Module 1: Project Structure for a Dioxus Workspace

A production-ready Dioxus fullstack project is best organized using a **Cargo Workspace**. This allows for clean separation of concerns, shared types, and efficient builds.

## Directory Layout
- `Cargo.toml` (Root): Defines the workspace members.
- `Dioxus.toml`: Configuration for the Dioxus CLI (Tailwind, platform-specific settings).
- `shared/`: Contains the data models and schemas used by both the frontend and backend. This ensures "Type Safety" across the entire stack.
- `backend/`: The Axum server. It handles the API routes, database interaction, and SSR.
- `frontend/`: The Dioxus UI. It contains the pages, components, and client-side logic.

## The `shared` Crate
The `shared` crate is the "source of truth". By defining our `ContentType` and `Entry` structs here once, we avoid duplicating code and ensure that when a model changes, both the frontend and backend are updated simultaneously.

## Benefits of this Structure
1. **Fast Compilation**: Cargo can compile crates in parallel.
2. **Simplified Refactoring**: Rust's type system catches API mismatches immediately.
3. **Multi-Platform Ready**: The frontend can be compiled for Web (WASM) or Desktop (WebView) while still using the same `shared` types.
