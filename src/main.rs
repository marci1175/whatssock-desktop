use core::panic::PanicMessage;

use dioxus::prelude::*;
use reqwest::Client;
use whatssock_desktop::api_requests::fetch_login;

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
    let mut client = use_signal(|| Client::new());
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
                        fetch_login(client(), username.to_string(), password.to_string()).await.unwrap();
                    });
                }, "Login" }
            }
        }
    }
}
