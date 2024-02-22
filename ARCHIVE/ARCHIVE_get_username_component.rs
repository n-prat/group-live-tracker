use gloo_storage::{LocalStorage, Storage};
use web_sys::console;
use yew::prelude::*;

use crate::websockets_common::update_username_in_context;

#[derive(Properties, PartialEq)]
pub struct UsernameFormProps {
    pub username: AttrValue,
    pub handle_oninput: Callback<Option<String>>,
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
struct LoginUserSchema {
    #[validate(
        length(min = 1, message = "username is required"),
        // email(message = "Email is invalid")
    )]
    email: String,
    // #[validate(
    //     length(min = 1, message = "Password is required"),
    //     length(min = 6, message = "Password must be at least 6 characters")
    // )]
    // password: String,
}

fn get_input_callback(
    name: &'static str,
    cloned_form: UseStateHandle<LoginUserSchema>,
) -> Callback<String> {
    Callback::from(move |value| {
        let mut data = cloned_form.deref().clone();
        match name {
            "username" => data.email = value,
            // "password" => data.password = value,
            _ => (),
        }
        cloned_form.set(data);
    })
}

/// see also https://github.com/wpcodevo/rust-yew-signup-signin/blob/62e9186ba1ede01b6d13eeeac036bbd56a131e1e/src/components/form_input.rs
#[function_component]
pub(crate) fn UsernameForm(props: &UsernameFormProps) -> Html {
    // Get a username from the Storage (ie cookies)
    let cookies_username: Option<String> = LocalStorage::get("username").ok();
    console::log_1(
        &format!(
            "UsernameForm create: username found in cookies: {:?}",
            cookies_username
        )
        .into(),
    );

    match &cookies_username {
        Some(username) => {
            // we have a valid username from cookies: nothing to display
            // TODO maybe display something in eg the top-right corner
            console::log_1(&format!("UsernameForm view: {:?}", username).into());

            update_username_in_context(username);

            html! {}
        }
        None => {
            // let oninput = Callback::from(move |input_event: InputEvent| {
            //     let new_char = input_event.data();
            //     username = match (&username, new_char) {
            //         // Trying to delete a char, but the current "self.username" is already blank
            //         (None, None) => &None,
            //         // standard case 1: we have a new input, and the current "username" is empty
            //         (None, Some(new_char)) => &Some(new_char),
            //         // Trying to delete a char
            //         (Some(current_username), None) => {
            //             let mut copy = current_username.clone();
            //             copy.pop();
            //             &Some(copy.clone())
            //         }
            //         // standard case 2: we have a new input, and the current "username" is NOT empty
            //         (Some(current_username), Some(new_char)) => {
            //             let mut copy = current_username.clone();
            //             copy.push_str(&new_char);
            //             &Some(copy.clone())
            //         }
            //     };
            // });

            // let onclick = Callback::from(move |mouse_event| {
            //     // Here you can handle the submission of the username
            //     // For example, you can send it to a server or store it locally
            //     // For simplicity, let's just print it to the console
            //     console::log_1(&format!("UsernameForm Msg::Submit: {:?}", username).into());

            //     let new_username = username.clone().expect("self.username NOT SET!");
            //     LocalStorage::set("username", new_username.clone())
            //         .expect("UsernameForm LocalStorage::set failed!");

            //     update_username_in_context(&new_username);
            // });

            let handle_username_input = get_input_callback("username", form.clone());

            // let oninput = Callback::from(move |input_event: InputEvent| {

            //     username = &AttrValue::Static("Hello");
            // });
            let handle_oninput = props.handle_oninput.clone();
            let oninput = Callback::from(move |input_event: InputEvent| {
                let new_char = input_event.data();
                console::log_1(&format!("UsernameForm new_char: {:?}", new_char).into());
                handle_oninput.emit(new_char);
            });

            let onclick = Callback::from(move |mouse_event: MouseEvent| {});

            html! {
                <div>
                    <input
                        type="text"
                        placeholder="Enter your username"
                        {oninput}
                    />
                    <button {onclick}>{"Submit"}</button>
                </div>
            }
        }
    }
}

// pub struct UsernameForm {
//     username: Option<String>,
// }

// pub enum Msg {
//     UpdateUsername(Option<String>),
//     Submit,
// }

// impl Component for UsernameForm {
//     type Message = Msg;
//     type Properties = ();

//     fn create(ctx: &Context<Self>) -> Self {

//         Self {
//             username: cookies_username,
//         }
//     }

//     fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
//         match msg {
//             Msg::UpdateUsername(new_username) => {

//                 false
//             }
//             Msg::Submit => {

//                 true
//             }
//         }
//     }

//     fn view(&self, ctx: &Context<Self>) -> Html {

//     }
// }
