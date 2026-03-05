use dioxus::prelude::*;

mod components;
mod pages;
mod services;

use pages::*;

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/login")]
    Login {},
    #[route("/signup")]
    Signup {},
    #[route("/dashboard")]
    Dashboard {},
    #[route("/content-types")]
    ContentTypes {},
    #[route("/entries/:slug")]
    Entries { slug: String },
}

#[server]
pub async fn get_server_status() -> Result<String, ServerFnError> {
    Ok("Connected to Dioxus Fullstack Server".to_string())
}

pub fn App() -> Element {
    let status = use_resource(get_server_status);

    rsx! {
        div { class: "min-h-screen bg-gray-50 text-gray-900 font-sans",
            div { class: "bg-blue-900 text-white text-xs py-1 px-4 flex justify-between",
                span { "Dioxus Fullstack CMS" }
                match &*status.read_unchecked() {
                    Some(Ok(s)) => rsx! { span { class: "text-green-400", "{s}" } },
                    _ => rsx! { span { class: "text-yellow-400", "Connecting..." } }
                }
            }
            Router::<Route> {}
        }
    }
}

#[cfg(feature = "web")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn start() {
    dioxus::launch(App);
}
