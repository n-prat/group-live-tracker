/// `https://github.com/wpcodevo/rust-yew-signup-signin/blob/62e9186ba1ede01b6d13eeeac036bbd56a131e1e/src/pages/login_page.rs`
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use crate::api::types::User;
use crate::api::user_api::api_login_user;
use crate::components::form_input::FormInput;
use crate::components::loading_button::LoadingButton;
use crate::router::{self};
use crate::store::{set_auth_user, set_page_loading, set_show_alert, PersistentStore, Store};

use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};
use wasm_bindgen_futures::spawn_local;
use web_sys::{console, HtmlInputElement};
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

#[derive(Validate, Debug, Default, Clone, Deserialize, Serialize)]
struct LoginUserSchema {
    #[validate(
        length(min = 1, message = "Username is required"),
        // email(message = "Email is invalid")
    )]
    email: String,
    // #[validate(
    //     length(min = 1, message = "Password is required"),
    //     length(min = 6, message = "Password must be at least 6 characters")
    // )]
    password: String,
}

fn get_input_callback(
    name: &'static str,
    cloned_form: UseStateHandle<LoginUserSchema>,
) -> Callback<String> {
    Callback::from(move |value| {
        let mut data = cloned_form.deref().clone();
        match name {
            "email" => data.email = value,
            "password" => data.password = value,
            _ => todo!("missing switch cf LoginUserSchema [1]!"),
        }
        cloned_form.set(data);
        console::debug_1(&format!("get_input_callback: cloned_form: {cloned_form:?}",).into());
    })
}

#[function_component(LoginPage)]
pub fn login_page() -> Html {
    let (store, dispatch) = use_store::<Store>();
    let (_persistent_store, dispatch2) = use_store::<PersistentStore>();
    let form = use_state(LoginUserSchema::default);
    let validation_errors = use_state(|| Rc::new(RefCell::new(ValidationErrors::new())));
    let navigator = use_navigator().unwrap();

    let email_input_ref = NodeRef::default();
    let password_input_ref = NodeRef::default();

    let validate_input_on_blur = {
        let cloned_form = form.clone();
        console::debug_1(
            &format!("login_page: validate_input_on_blur: cloned_form: {cloned_form:?}",).into(),
        );
        let cloned_validation_errors = validation_errors.clone();
        Callback::from(move |(name, value): (String, String)| {
            let mut data = cloned_form.deref().clone();
            match name.as_str() {
                "email" => data.email = value,
                "password" => data.password = value,
                _ => todo!("missing switch cf LoginUserSchema [2]!"),
            }
            cloned_form.set(data);

            console::debug_1(
                &format!(
                    "login_page: validate_input_on_blur: Callback: cloned_form: {cloned_form:?}",
                )
                .into(),
            );

            match cloned_form.validate() {
                Ok(()) => {
                    cloned_validation_errors
                        .borrow_mut()
                        .errors_mut()
                        .remove(name.as_str());
                }
                Err(errors) => {
                    cloned_validation_errors
                        .borrow_mut()
                        .errors_mut()
                        .retain(|key, _| key != &name);
                    for (field_name, error) in errors.errors() {
                        if field_name == &name {
                            cloned_validation_errors
                                .borrow_mut()
                                .errors_mut()
                                .insert(field_name, error.clone());
                        }
                    }
                }
            }
        })
    };

    let handle_email_input = get_input_callback("email", form.clone());
    let handle_password_input = get_input_callback("password", form.clone());

    let on_submit = {
        console::debug_1(&"login_page: on_submit".into());
        let cloned_form = form.clone();
        console::debug_1(&format!("login_page: on_submit: cloned_form: {cloned_form:?}",).into());
        let cloned_validation_errors = validation_errors.clone();
        let store_dispatch = dispatch.clone();
        let store_dispatch2 = dispatch2.clone();
        let cloned_navigator = navigator.clone();

        let cloned_email_input_ref = email_input_ref.clone();
        let cloned_password_input_ref = password_input_ref.clone();

        Callback::from(move |event: SubmitEvent| {
            console::debug_1(&"login_page: on_submit Callback".into());
            event.prevent_default();

            let dispatch = store_dispatch.clone();
            let dispatch2 = store_dispatch2.clone();
            let form = cloned_form.clone();
            let validation_errors = cloned_validation_errors.clone();
            let navigator = cloned_navigator.clone();

            let email_input_ref = cloned_email_input_ref.clone();
            let password_input_ref = cloned_password_input_ref.clone();

            spawn_local(async move {
                console::debug_1(&"login_page: on_submit Callback spawn_local".into());
                match form.validate() {
                    Ok(()) => {
                        let form_data = form.deref().clone();
                        set_page_loading(true, &dispatch);

                        let email_input = email_input_ref.cast::<HtmlInputElement>().unwrap();
                        let password_input = password_input_ref.cast::<HtmlInputElement>().unwrap();

                        email_input.set_value("");
                        password_input.set_value("");

                        let form_json = serde_json::to_string(&form_data).unwrap();
                        console::debug_1(&"login_page: on_submit Callback form_json".into());
                        let res = api_login_user(&form_json).await;
                        match res {
                            Ok(res) => {
                                set_page_loading(false, &dispatch);
                                set_auth_user(
                                    Some(User {
                                        username: form_data.email,
                                        is_super_user: false,
                                    }),
                                    Some(res.access_token),
                                    &dispatch2,
                                );
                                navigator.push(&router::Route::HomePage);
                            }
                            Err(e) => {
                                set_page_loading(false, &dispatch);
                                set_show_alert(e.to_string(), &dispatch);
                            }
                        };
                    }
                    Err(e) => {
                        validation_errors.set(Rc::new(RefCell::new(e)));
                    }
                }
            });
        })
    };

    html! {
    <section class="bg-ct-blue-600 min-h-screen grid place-items-center">
      <div class="w-full">
        <h1 class="text-4xl xl:text-6xl text-center font-[600] text-ct-yellow-600 mb-4">
          {"Welcome Back"}
        </h1>
        <h2 class="text-lg text-center mb-4 text-ct-dark-200">
          {"Login to have access"}
        </h2>
          <form
            // TODO onsubmit? why is this not called?
            onsubmit={on_submit}
            class="max-w-md w-full mx-auto overflow-hidden shadow-lg bg-ct-dark-200 rounded-2xl p-8 space-y-5"
          >
            <FormInput label="Email" name="email" input_type="text" input_ref={email_input_ref} handle_onchange={handle_email_input} errors={&*validation_errors} handle_on_input_blur={validate_input_on_blur.clone()} />
            <FormInput label="Password" name="password" input_type="password" input_ref={password_input_ref} handle_onchange={handle_password_input} errors={&*validation_errors} handle_on_input_blur={validate_input_on_blur.clone()}/>

            <div class="text-right">
              <a href="#">
                {"Forgot Password?"}
              </a>
            </div>

            <LoadingButton
              loading={store.page_loading}
              text_color={Some("text-ct-blue-600".to_string())}
            >
              {"Login"}
            </LoadingButton>

            // TODO
            // <span class="block">
            //   {"Need an account?"} {" "}
            //   <Link<Route> to={Route::RegisterPage} classes="text-ct-blue-600">{ "Sign Up Here" }</Link<Route>>
            // </span>
          </form>
      </div>
    </section>
    }
}
