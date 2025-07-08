use std::{fmt::Display, sync::Arc};

use crate::HttpClient;
use dioxus::{logger::tracing, prelude::*};
use parking_lot::Mutex;
use reqwest::{Client, StatusCode};

enum AttemptResult {
    Attempted(String),
    Succeeded(String),
    Failed(String),
}

impl Display for AttemptResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            AttemptResult::Attempted(inner) => inner,
            AttemptResult::Succeeded(inner) => inner,
            AttemptResult::Failed(inner) => inner,
        })
    }
}

#[component]
pub fn Register() -> Element {
    let client = Arc::new(Mutex::new(
        HttpClient::new(Client::new(), {
            #[cfg(debug_assertions)]
            {
                String::from("http://[::1]:3004")
            }
            #[cfg(not(debug_assertions))]
            {
                String::from("http://whatssock.com")
            }
        })
    ));

    let mut log_res: Signal<Option<AttemptResult>> = use_signal(|| None);

    let mut username = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut email = use_signal(String::new);

    rsx! {
        div {
            id: "register_page_container",
            div {
                id: "user_input_fields",

                div {
                    id: "username_field",
                    input {
                        oninput: move |event| username.set(event.value()),
                        placeholder: "Username",
                    }
                }

                div {
                    id: "password_field",
                    input {
                        oninput: move |event| email.set(event.value()),
                        placeholder: "Email",
                    }
                }

                div {
                    id: "password_field",
                    input {
                        oninput: move |event| password.set(event.value()),
                        placeholder: "Password",
                        r#type: "password",
                    }
                }

                button { onclick: move |_| {
                    // Update state
                    log_res.set(Some(AttemptResult::Attempted("Registering...".to_string())));

                    let client = client.clone();

                    // Spawn async task
                    spawn(async move {
                        match client.lock().send_register_request(username.to_string(), password.to_string(), email.to_string()).await {
                            Ok(response) => {
                                tracing::info!("{}", response.text().await.unwrap());

                                // Update state
                                log_res.set(Some(AttemptResult::Succeeded("Register Successful!".to_string())));
                            },
                            Err(err) => {
                                tracing::error!("Error occured when registering: {}", err.to_string());

                                // Update state
                                log_res.set(Some(AttemptResult::Failed(err.to_string())));
                            },
                        }
                    });
                }, "Register" }

                // Check if there is an existing error message
                div {
                    id: "register_result",
                    {
                        if let Some(register_result) = &*log_res.read() {
                            // Display the result
                            match register_result {
                                AttemptResult::Attempted(inner) => {
                                    rsx! {
                                        div {
                                            id: "attempted",
                                            {
                                                inner.to_string()
                                            }
                                        }
                                    }
                                },
                                AttemptResult::Succeeded(inner) => {
                                    rsx! {
                                        div {
                                            id: "succeeded",
                                            {
                                                inner.to_string()
                                            }
                                        }
                                    }
                                },
                                AttemptResult::Failed(inner) => {
                                    rsx! {
                                        div {
                                            id: "failed",
                                            {
                                                inner.to_string()
                                            }
                                        }
                                    }
                                },
                            }
                        }
                        else {
                            rsx!()
                        }
                    }
                }
            }
        }

        ErrorBoundary {
            handle_error: |_| {
                rsx! {
                    div {
                        "Oops, we encountered an error. Please report this to the developer of this application"
                    }
                }
            }
        }
    }
}
