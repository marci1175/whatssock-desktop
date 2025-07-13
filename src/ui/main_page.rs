use std::sync::Arc;

use dioxus::prelude::*;
use dioxus_toast::{ToastInfo, ToastManager};
use parking_lot::Mutex;

use crate::{authentication::UserSession, HttpClient, Route, UserInformation};

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct ChatEntry {
    title: String,
    last_message: String,
    last_message_date: chrono::NaiveDate,
    channel_icon_url: String,
}

impl ChatEntry {
    pub fn new(
        title: String,
        last_message: String,
        last_message_date: chrono::NaiveDate,
        channel_icon_url: String,
    ) -> Self {
        Self {
            title,
            last_message,
            last_message_date,
            channel_icon_url,
        }
    }
}

#[component]
pub fn MainPage() -> Element {
    let (user_session, user_information) = use_context::<(UserSession, UserInformation)>();
    let client = use_context::<Arc<Mutex<HttpClient>>>();
    let mut navigator = navigator();
    let mut user_chat_entries: Signal<Vec<ChatEntry>> = use_signal(|| Vec::new());
    let mut toast: Signal<ToastManager> = use_context();
    user_chat_entries.set(vec![
        ChatEntry::new(
            "Muslincák".to_string(),
            "marci: kurva anyad".to_string(),
            chrono::Local::now().date_naive(),
            "channel_icon_url".to_string(),
        ),
        ChatEntry::new(
            "Muslincák".to_string(),
            "marci: kurva anyad".to_string(),
            chrono::Local::now().date_naive(),
            "channel_icon_url".to_string(),
        ),
        ChatEntry::new(
            "Muslincák".to_string(),
            "marci: kurva anyad".to_string(),
            chrono::Local::now().date_naive(),
            "channel_icon_url".to_string(),
        ),
        ChatEntry::new(
            "Muslincák".to_string(),
            "marci: kurva anyad".to_string(),
            chrono::Local::now().date_naive(),
            "channel_icon_url".to_string(),
        ),
        ChatEntry::new(
            "Muslincák".to_string(),
            "marci: kurva anyad".to_string(),
            chrono::Local::now().date_naive(),
            "channel_icon_url".to_string(),
        ),
        ChatEntry::new(
            "Muslincák".to_string(),
            "marci: kurva anyad".to_string(),
            chrono::Local::now().date_naive(),
            "channel_icon_url".to_string(),
        ),
        ChatEntry::new(
            "Muslincák".to_string(),
            "marci: kurva anyad".to_string(),
            chrono::Local::now().date_naive(),
            "channel_icon_url".to_string(),
        ),
        ChatEntry::new(
            "Muslincák".to_string(),
            "marci: kurva anyad".to_string(),
            chrono::Local::now().date_naive(),
            "channel_icon_url".to_string(),
        ),
    ]);

    rsx! {
        div {
            class: "sidepanel",
            id: "sidepanel_left",

            div {
                id: "sidepanel_title",
                class: "title",
                "Current Messages"
            }

            div {
                id: "chat_entry_list",

                for chat_entry in dbg!(user_chat_entries()) {
                button {
                    id: "chat_entry",

                    div {
                        id: "chat_icon",
                        img {}
                    }

                    div {
                        id: "chat_entry_title",

                        div {
                            {
                                chat_entry.title
                            }
                        }

                        div { {
                            chat_entry.last_message_date.to_string()
                        } }
                    }

                    div {
                        id: "chat_entry_last_message",

                        {
                            chat_entry.last_message
                        }
                    }
                }
            }
            }
            
            div {
                id: "user_control_panel_area",
                {
                    format!("Logged in as: {}", user_information.username)
                }

                div {
                    id: "user_control_panel_buttons",
                    button {
                        id: "user_control_panel_button",
                        "Settings"
                    }
                    
                    button {
                        id: "user_control_panel_button",
                        onclick: move |_event| {
                            let client = client.clone();
                            let user_session = user_session.clone();

                            spawn(async move {
                                client.lock().request_logout(user_session.clone()).await.unwrap();
                            });
                            
                            toast.write().popup(ToastInfo::simple("Successfully logged out!"));
                            
                            navigator.replace(Route::Login {  });
                        },

                        "Logout"
                    }
                }
            }
        }

        div {
            class: "default_panel",
            id: "chatpanel",
        }
    }
}
