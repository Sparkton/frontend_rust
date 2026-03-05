# Module 2: Server-Side Rendering (SSR) vs. Hydration in Dioxus

Dioxus Fullstack uses a powerful combination of SSR and Hydration to provide the best user experience and performance.

## Server-Side Rendering (SSR)
When a user first requests a page:
1. The **Axum server** receives the HTTP request.
2. It runs the Dioxus `App()` component on the server.
3. It renders the component tree into a static **HTML string**.
4. It sends this HTML to the browser immediately.

**Benefit**: Fast "Time to First Byte" and excellent SEO.

## Hydration
After the browser receives and displays the initial HTML:
1. It downloads the **WASM bundle** (the client-side frontend code).
2. The WASM "Hydrates" the static HTML.
3. This process connects the event listeners (like `onclick`) and state management to the existing DOM elements.

**Result**: The static page becomes a fully interactive Single-Page App (SPA).

## Production Workflow
In production, `dioxus-cli` bundles the WASM and assets into a `public/` directory. The Axum server is then configured to:
- Serve static assets from `public/`.
- Handle specific Dioxus Fullstack routes.
- Handle standard API requests (`/api/...`).
