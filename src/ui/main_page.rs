use dioxus::prelude::*;

use crate::authentication::UserSession;

#[component]
pub fn MainPage() -> Element {
    let user_session = use_context::<UserSession>();
    
    rsx! {
        {
            format!("{user_session:?}")
        }
    }
}
