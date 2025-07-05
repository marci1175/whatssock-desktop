use std::{format, path::PathBuf};

use dioxus::prelude::*;
use whatssock_desktop::ui::{login::Login, main_page::MainPage};

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() -> anyhow::Result<()> {
    // Initalize a local file for in the user's folder
    let path = PathBuf::from(format!("{}/.whatssock", std::env::var("USERPROFILE").unwrap()));
    std::fs::create_dir(path)?;
    
    dioxus::launch(App);
    
    Ok(())
}

#[derive(Routable, PartialEq, Clone)]
enum Route {
    #[route("/")]
    Login {},
    #[route("/chats")]
    MainPage {},
    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}

#[component]
fn NotFound(segments: Vec<String>) -> Element {
    rsx! {
        div {
            id: "not_found_page",
            {
                format!(r#"Path "{:?}" is not found."#, segments)
            }
        }
    }
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
    }
}
