/// https://github.com/wpcodevo/rust-yew-signup-signin/blob/62e9186ba1ede01b6d13eeeac036bbd56a131e1e/src/components/header.rs
///
use crate::{
    api::user_api::api_logout_user,
    router::{self, Route},
    store::{set_auth_user, set_page_loading, set_show_alert, PersistentStore, Store},
};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

#[function_component(Header)]
pub fn header_component() -> Html {
    let (store, dispatch) = use_store::<Store>();
    let (persistent_store, dispatch2) = use_store::<PersistentStore>();
    let user = persistent_store.auth_user.clone();
    let navigator = use_navigator().unwrap();

    let handle_logout = {
        let store_dispatch = dispatch.clone();
        let store_dispatch2 = dispatch2.clone();
        let cloned_navigator = navigator.clone();

        Callback::from(move |_: MouseEvent| {
            let dispatch = store_dispatch.clone();
            let dispatch2 = store_dispatch2.clone();
            let navigator = cloned_navigator.clone();
            spawn_local(async move {
                set_page_loading(true, dispatch.clone());
                let res = api_logout_user().await;
                match res {
                    Ok(_) => {
                        set_page_loading(false, dispatch.clone());
                        set_auth_user(None, dispatch2.clone());
                        set_show_alert("Logged out successfully".to_string(), dispatch);
                        navigator.push(&router::Route::LoginPage);
                    }
                    Err(e) => {
                        set_show_alert(e.to_string(), dispatch.clone());
                        set_page_loading(false, dispatch);
                    }
                };
            });
        })
    };

    html! {
        <header class="bg-white h-20">
        <nav class="h-full flex justify-between container items-center">
          <div>
            <Link<Route> to={Route::HomePage} classes="text-ct-dark-600">{"group_live_tracker"}</Link<Route>>
          </div>
          <ul class="flex items-center gap-4">
            <li>
              <Link<Route> to={Route::HomePage} classes="text-ct-dark-600">{"Home"}</Link<Route>>
            </li>
            if user.is_some() {
               <>
                <li>
                  // TODO
                  // <Link<Route> to={Route::ProfilePage} classes="text-ct-dark-600">{"Profile"}</Link<Route>>
                </li>
                <li
                  class="cursor-pointer"
                >
                  {"Create Post"}
                </li>
                <li class="cursor-pointer" onclick={handle_logout}>
                  {"Logout"}
                </li>
              </>

            } else {
              <>
                <li>
                  // TODO
                  // <Link<Route> to={Route::RegisterPage} classes="text-ct-dark-600">{"SignUp"}</Link<Route>>
                </li>
                <li>
                  <Link<Route> to={Route::LoginPage} classes="text-ct-dark-600">{"Login"}</Link<Route>>
                </li>
              </>
            }
          </ul>
        </nav>
      </header>
    }
}
