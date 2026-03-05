use dioxus::prelude::*;
use crate::Route;
use shared::{ContentType, CMSSchema, FieldDefinition, FieldType};
use crate::components::DynamicForm;
use crate::services;

#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "p-8 max-w-4xl mx-auto text-center py-20",
            h1 { class: "text-5xl font-extrabold mb-6 bg-clip-text text-transparent bg-gradient-to-r from-blue-600 to-indigo-600", "Headless CMS" }
            p { class: "text-xl text-gray-600 mb-10", "Built with Dioxus, Axum, and SurrealDB." }
            div { class: "flex justify-center space-x-6",
                Link { to: Route::Login {}, class: "bg-blue-600 text-white px-8 py-3 rounded-lg font-bold shadow-lg hover:bg-blue-700 transition", "Login" }
                Link { to: Route::Signup {}, class: "bg-white text-gray-800 px-8 py-3 rounded-lg font-bold shadow border border-gray-200 hover:bg-gray-50 transition", "Sign Up" }
            }
        }
    }
}

#[component]
pub fn Login() -> Element {
    let mut username = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut error = use_signal(|| None::<String>);
    let nav = use_navigator();

    rsx! {
        div { class: "flex flex-col items-center justify-center h-screen bg-gray-50",
            div { class: "bg-white p-8 rounded shadow-md w-96",
                h2 { class: "text-2xl font-bold mb-6 text-center", "CMS Login" }
                if let Some(err) = error.read().as_ref() {
                    p { class: "text-red-500 mb-4 text-center text-sm", "{err}" }
                }
                div { class: "space-y-4",
                    div {
                        label { class: "block text-sm font-medium text-gray-700", "Username" }
                        input {
                            type: "text",
                            value: "{username}",
                            oninput: move |e| username.set(e.value()),
                            class: "mt-1 block w-full border border-gray-300 rounded-md shadow-sm p-2"
                        }
                    }
                    div {
                        label { class: "block text-sm font-medium text-gray-700", "Password" }
                        input {
                            type: "password",
                            value: "{password}",
                            oninput: move |e| password.set(e.value()),
                            class: "mt-1 block w-full border border-gray-300 rounded-md shadow-sm p-2"
                        }
                    }
                    button {
                        onclick: move |_| {
                            let user = username.read().clone();
                            let pass = password.read().clone();
                            spawn(async move {
                                match services::login(user, pass).await {
                                    Ok(_) => { nav.push(Route::Dashboard {}); },
                                    Err(e) => { error.set(Some(e)); },
                                }
                            });
                        },
                        class: "w-full bg-blue-600 text-white px-4 py-2 rounded-md hover:bg-blue-700 transition",
                        "Sign In"
                    }
                }
            }
        }
    }
}

#[component]
pub fn Signup() -> Element {
    rsx! {
        div { class: "flex flex-col items-center justify-center h-screen",
            h2 { class: "text-2xl font-bold mb-4", "Sign Up" }
            p { "Sign up is managed by administrators in this CMS." }
            Link { to: Route::Login {}, class: "text-blue-600 mt-4", "Back to Login" }
        }
    }
}

#[component]
pub fn Dashboard() -> Element {
    rsx! {
        div { class: "p-8 max-w-4xl mx-auto",
            h1 { class: "text-3xl font-bold mb-8", "CMS Dashboard" }
            div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                Link { to: Route::ContentTypes {}, class: "bg-white p-6 rounded-lg shadow border border-gray-100 hover:shadow-lg transition",
                    h3 { class: "text-xl font-bold text-blue-600 mb-2", "Content Types" }
                    p { class: "text-gray-600", "Define the structure of your data (e.g., Blog Posts, Products)." }
                }
                div { class: "bg-white p-6 rounded-lg shadow border border-gray-100 opacity-50 cursor-not-allowed",
                    h3 { class: "text-xl font-bold text-gray-400 mb-2", "Media Library" }
                    p { class: "text-gray-600", "Manage uploaded images and files." }
                }
            }
        }
    }
}

#[component]
pub fn ContentTypes() -> Element {
    let content_types = use_resource(services::fetch_content_types);

    rsx! {
        div { class: "p-8 max-w-4xl mx-auto",
            div { class: "flex justify-between items-center mb-8",
                h1 { class: "text-3xl font-bold", "Content Types" }
                button { class: "bg-blue-600 text-white px-4 py-2 rounded", "New Content Type" }
            }

            match &*content_types.read_unchecked() {
                Some(cts) => rsx! {
                    div { class: "space-y-4",
                        for ct in cts {
                            div { class: "bg-white p-4 rounded shadow border border-gray-100 flex justify-between items-center",
                                div {
                                    h3 { class: "font-bold text-lg", "{ct.name}" }
                                    p { class: "text-sm text-gray-500", "{ct.slug}" }
                                }
                                Link { to: Route::Entries { slug: ct.slug.clone() }, class: "text-blue-600 hover:underline", "View Entries" }
                            }
                        }
                        if cts.is_empty() {
                            p { class: "text-gray-500 text-center py-8", "No content types defined yet." }
                        }
                    }
                },
                None => rsx! { p { "Loading..." } }
            }
        }
    }
}

#[component]
pub fn Entries(slug: String) -> Element {
    let entries = use_resource({
        let slug = slug.clone();
        move || services::fetch_entries(slug.clone())
    });

    rsx! {
        div { class: "p-8 max-w-4xl mx-auto",
            h1 { class: "text-3xl font-bold mb-8", "Entries: {slug}" }

            div { class: "grid grid-cols-1 gap-8",
                div { class: "bg-white p-6 rounded-lg shadow",
                    h2 { class: "text-xl font-bold mb-4 border-b pb-2", "Add New Entry" }
                    DynamicForm {
                        schema: CMSSchema {
                            content_type: ContentType {
                                id: None,
                                name: "New Entry".into(),
                                slug: slug.clone(),
                                fields: vec![
                                    FieldDefinition { name: "Title".into(), field_type: FieldType::ShortText, required: true, help_text: None },
                                    FieldDefinition { name: "Image".into(), field_type: FieldType::Image, required: false, help_text: None },
                                ]
                            }
                        },
                        on_submit: move |data| {
                            println!("New entry data: {:?}", data);
                        }
                    }
                }

                div { class: "mt-8",
                    h2 { class: "text-xl font-bold mb-4", "Existing Entries" }
                    match &*entries.read_unchecked() {
                        Some(items) => rsx! {
                            div { class: "space-y-2",
                                for item in items {
                                    div { class: "p-3 bg-gray-50 border rounded",
                                        "{item.id:?}"
                                    }
                                }
                                if items.is_empty() {
                                    p { class: "text-gray-500", "No entries yet." }
                                }
                            }
                        },
                        None => rsx! { p { "Loading..." } }
                    }
                }
            }
        }
    }
}
