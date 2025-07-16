use std::sync::Arc;

use dioxus::prelude::*;
use dioxus_toast::{ToastInfo, ToastManager};
use parking_lot::Mutex;

use crate::{
    authentication::{FetchChatroomRequest, FetchChatroomResponse, UserSession},
    HttpClient, Route, UserInformation,
};


#[component]
pub fn MainPage() -> Element {
    let (user_session, user_information) = use_context::<(UserSession, UserInformation)>();
    let mut toast: Signal<ToastManager> = use_context();

    let user_session = Arc::new(user_session);
    let user_session_clone = user_session.clone();

    let client = use_context::<Arc<Mutex<HttpClient>>>();
    let client_clone = client.clone();

    let navigator = navigator();
    let mut user_chat_entries: Signal<Vec<FetchChatroomResponse>> = use_signal(|| Vec::new());

    let mut chatroom_id_buffer = use_signal(|| String::new());
    let mut chatroom_passw_buffer = use_signal(|| String::new());

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

                for chat_entry in user_chat_entries.read().clone() {
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
                                    chat_entry.chatroom_name
                                }
                            }

                            // div { {
                            //     chat_entry.last_message_date.to_string()
                            // } }
                        }

                        div {
                            id: "chat_entry_last_message",

                            {
                                format!("{:?}", chat_entry.last_message_id)
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
                                // Send the logout request
                                client.lock().request_logout(user_session.clone()).await.unwrap();

                                // Reset root ctx for the session
                                let mut session_ctx = use_context::<Signal<Option<(UserSession, UserInformation)>>>();
                                session_ctx.set(None);

                                navigator.replace(Route::Login {  });

                                // Push the notification
                                toast.write().popup(ToastInfo::simple("Successfully logged out!"));
                            });
                        },

                        "Logout"
                    }
                div {
                "Add a new chat!"
            }
            div {
                id: "chat_id_input_row",

                button {
                    id: "new_chat_button",
                    class: "button",
                    onclick: move |_| {
                        let client = client_clone.clone();
                        let user_session = user_session_clone.clone();

                        spawn(async move {
                            let response = client.lock().fetch_chatroom_id(FetchChatroomRequest { user_session: (*user_session).clone(), chatroom_id: chatroom_id_buffer.to_string(), password: {
                                let passw_str = chatroom_passw_buffer.to_string();

                                if passw_str.is_empty() {
                                    None
                                }
                                else {
                                    Some(passw_str)
                                }
                            } }).await.unwrap();

                            let serialized_response = serde_json::from_str::<FetchChatroomResponse>(&response.text().await.unwrap()).unwrap();

                            user_chat_entries.push(serialized_response);
                        });
                    },

                    "+"
                }
                input {
                    oninput: move |event| {
                        chatroom_id_buffer.set(event.value());
                    },
                    placeholder: "Chat ID",
                }
            }
            input {
                oninput: move |event| {
                    chatroom_passw_buffer.set(event.value());
                },
                placeholder: "Chat Password",
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
