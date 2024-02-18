use gloo_storage::{LocalStorage, Storage};
use web_sys::console;
use yew::prelude::*;

use crate::websockets_common::update_username_in_context;

pub struct UsernameForm {
    username: Option<String>,
}

pub enum Msg {
    UpdateUsername(Option<String>),
    Submit,
}

impl Component for UsernameForm {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        // Get a username from the Storage (ie cookies)
        let cookies_username: Option<String> = LocalStorage::get("username").ok();
        console::log_1(
            &format!(
                "UsernameForm create: username found in cookies: {:?}",
                cookies_username
            )
            .into(),
        );

        if let Some(username) = &cookies_username {
            update_username_in_context(ctx, username);
        }

        Self {
            username: cookies_username,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateUsername(new_username) => {
                self.username = match (&self.username, new_username) {
                    // Trying to delete a char, but the current "self.username" is already blank
                    (None, None) => None,
                    // standard case 1: we have a new input, and the current "self.username" is empty
                    (None, Some(new_username)) => Some(new_username),
                    // Trying to delete a char
                    (Some(current_username), None) => {
                        let mut copy = current_username.clone();
                        copy.pop();
                        Some(copy.clone())
                    }
                    // standard case 2: we have a new input, and the current "self.username" is NOT empty
                    (Some(current_username), Some(new_username)) => {
                        let mut copy = current_username.clone();
                        copy.push_str(&new_username);
                        Some(copy.clone())
                    }
                };

                false
            }
            Msg::Submit => {
                // Here you can handle the submission of the username
                // For example, you can send it to a server or store it locally
                // For simplicity, let's just print it to the console
                console::log_1(&format!("UsernameForm Msg::Submit: {:?}", self.username).into());

                let new_username = self.username.clone().expect("self.username NOT SET!");
                LocalStorage::set("username", new_username.clone())
                    .expect("UsernameForm LocalStorage::set failed!");

                update_username_in_context(ctx, &new_username);

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match &self.username {
            Some(username) => {
                // we have a valid username from cookies: nothing to display
                // TODO maybe display something in eg the top-right corner
                console::log_1(&format!("UsernameForm view: {:?}", username).into());
                html! {}
            }
            None => {
                html! {
                    <div>
                        <input
                            type="text"
                            placeholder="Enter your username"
                            oninput={ctx.link().callback(|e: InputEvent| Msg::UpdateUsername(e.data()))}
                        />
                        <button onclick={ctx.link().callback(|_e| Msg::Submit)}>{"Submit"}</button>
                    </div>
                }
            }
        }
    }
}
