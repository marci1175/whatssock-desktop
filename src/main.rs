use std::{format, path::PathBuf};

use dioxus::prelude::*;
use whatssock_desktop::{ui::{login::Login, main_page::MainPage, register::Register}, Route};

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() -> anyhow::Result<()> {
    // Initalize a local file for in the user's folder
    let path = PathBuf::from(format!("{}/.whatssock", std::env::var("USERPROFILE").unwrap()));

    // Only attempt to create the folder if it doesnt exist yet
    if let Err(err) = std::fs::read_dir(&path) {
        std::fs::create_dir(path)?;
    }
    
    dioxus::launch(App);
    
    Ok(())
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
    }
}
