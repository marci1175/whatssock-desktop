use core::panic::PanicMessage;
use std::format;

use dioxus::{
    logger::{self, tracing},
    prelude::*,
};
use reqwest::Client;
use whatssock_desktop::HttpClient;

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        LoginPage {}
    }
}

#[component]
pub fn LoginPage() -> Element {
    let client = use_signal(|| {
        HttpClient::new(Client::new(), {
            #[cfg(debug_assertions)]
            {
                String::from("[::1]")
            }
            #[cfg(not(debug_assertions))]
            {
                String::from("whatssock.com")
            }
        })
    });

    let mut last_login_attempt_result: Signal<Option<String>> = use_signal(|| None);

    let mut username = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    rsx! {
        div {
            id: "login_page_container",
            div {
                id: "main_title",
                "Whatssock"
            }

            footer {
                id: "main_title_footer",
                "Wassup? Interact with others now."
            }

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
                        oninput: move |event| password.set(event.value()),
                        placeholder: "Password",
                        r#type: "password",
                    }
                }

                button { onclick: move |event| {
                    use_future(move || async move {
                        match client.read().fetch_login(username.to_string(), password.to_string()).await {
                            Ok(response) => {
                                tracing::info!("{}", response.text().await.unwrap())
                            },
                            Err(err) => {
                                tracing::error!("Error occured when logging in: {}", err.to_string());

                                last_login_attempt_result.set(Some(err.to_string()));

                                return Err(err);    
                            },
                        }

                        Ok(())
                    });
                }, "Login" }

                {
                    if let Some(error_msg) = dbg!(&*last_login_attempt_result.read()) {
                        rsx! {
                            text {
                                id: "login_error",
                                {format!("An error occured while attempting to login: {}", error_msg)}
                            }
                        }
                    }
                    else {
                        rsx!()
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
