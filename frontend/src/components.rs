use dioxus::prelude::*;
use shared::{FieldType, CMSSchema};
use std::collections::HashMap;

#[component]
pub fn DynamicForm(schema: CMSSchema, on_submit: EventHandler<HashMap<String, serde_json::Value>>) -> Element {
    let mut form_data = use_signal(HashMap::<String, serde_json::Value>::new);

    rsx! {
        form {
            onsubmit: move |_| {
                on_submit.call(form_data.cloned());
            },
            class: "space-y-4",
            for field in schema.content_type.fields {
                div { class: "flex flex-col",
                    label { class: "mb-2 font-medium text-gray-700", "{field.name}" }
                    match field.field_type {
                        FieldType::ShortText => {
                            let field_name = field.name.clone();
                            rsx! {
                                input {
                                    type: "text",
                                    required: field.required,
                                    oninput: move |evt| {
                                        form_data.with_mut(|fd| {
                                            fd.insert(field_name.clone(), serde_json::Value::String(evt.value()));
                                        });
                                    },
                                    class: "border rounded-md p-2 shadow-sm focus:ring-blue-500 focus:border-blue-500"
                                }
                            }
                        },
                        FieldType::LongText => {
                            let field_name = field.name.clone();
                            rsx! {
                                textarea {
                                    required: field.required,
                                    oninput: move |evt| {
                                        form_data.with_mut(|fd| {
                                            fd.insert(field_name.clone(), serde_json::Value::String(evt.value()));
                                        });
                                    },
                                    class: "border rounded-md p-2 shadow-sm focus:ring-blue-500 focus:border-blue-500 h-24"
                                }
                            }
                        },
                        FieldType::Number => {
                            let field_name = field.name.clone();
                            rsx! {
                                input {
                                    type: "number",
                                    required: field.required,
                                    oninput: move |evt| {
                                        if let Ok(val) = evt.value().parse::<f64>() {
                                            form_data.with_mut(|fd| {
                                                fd.insert(field_name.clone(), serde_json::json!(val));
                                            });
                                        }
                                    },
                                    class: "border rounded-md p-2 shadow-sm focus:ring-blue-500 focus:border-blue-500"
                                }
                            }
                        },
                        FieldType::Boolean => {
                            let field_name = field.name.clone();
                            rsx! {
                                input {
                                    type: "checkbox",
                                    required: field.required,
                                    onchange: move |evt| {
                                        form_data.with_mut(|fd| {
                                            fd.insert(field_name.clone(), serde_json::json!(evt.value() == "true"));
                                        });
                                    },
                                    class: "h-4 w-4 text-blue-600 border-gray-300 rounded focus:ring-blue-500"
                                }
                            }
                        },
                        FieldType::Date => {
                            let field_name = field.name.clone();
                            rsx! {
                                input {
                                    type: "date",
                                    required: field.required,
                                    oninput: move |evt| {
                                        form_data.with_mut(|fd| {
                                            fd.insert(field_name.clone(), serde_json::Value::String(evt.value()));
                                        });
                                    },
                                    class: "border rounded-md p-2 shadow-sm focus:ring-blue-500 focus:border-blue-500"
                                }
                            }
                        },
                        FieldType::Image => {
                            let field_name = field.name.clone();
                            rsx! {
                                input {
                                    type: "file",
                                    accept: "image/*",
                                    onchange: move |evt| {
                                        form_data.with_mut(|fd| {
                                            fd.insert(field_name.clone(), serde_json::json!(evt.value()));
                                        });
                                    },
                                    class: "border border-dashed border-gray-300 rounded-md p-4 text-sm text-gray-500"
                                }
                            }
                        },
                        FieldType::Relation(ref target) => {
                            let field_name = field.name.clone();
                            let target = target.clone();
                            rsx! {
                                select {
                                    required: field.required,
                                    onchange: move |evt| {
                                        form_data.with_mut(|fd| {
                                            fd.insert(field_name.clone(), serde_json::Value::String(evt.value()));
                                        });
                                    },
                                    class: "border rounded-md p-2 shadow-sm focus:ring-blue-500 focus:border-blue-500",
                                    option { value: "", "Select {target}..." }
                                    option { value: "id-1", "Mock {target} Item 1" }
                                    option { value: "id-2", "Mock {target} Item 2" }
                                }
                            }
                        }
                    }
                }
            }
            button {
                type: "submit",
                class: "mt-4 w-full bg-blue-600 text-white px-4 py-2 rounded-md hover:bg-blue-700 transition font-medium",
                "Save Entry"
            }
        }
    }
}
