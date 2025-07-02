use core::panic::PanicMessage;

use dioxus::prelude::*;

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
                        placeholder: "Username",
                    }
                }

                div {  
                    id: "password_field",
                    input {
                        placeholder: "Password",
                        r#type: "password",
                    }
                }
            }
        }
    }
}
