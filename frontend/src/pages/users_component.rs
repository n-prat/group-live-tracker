use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// `https://github.com/slowtec/leaflet-rs/blob/master/examples/yew-component/src/components/map_component.rs`
use gloo_utils::document;
use js_sys::Array;
use leaflet::{Circle, LatLng, Map, MapOptions, Polyline, PolylineOptions, TileLayer};
use leaflet::{Tooltip, TooltipOptions};
use serde_json::Value;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::spawn_local;
use web_sys::{console, HtmlElement};
use yew::prelude::*;
use yew_hooks::{use_async, use_async_with_options, UseAsyncOptions};
use yewdux::use_store;

use crate::api::types::ListUsers;
use crate::api::user_api::api_list_users;
use crate::store::{set_page_loading, set_show_alert, PersistentStore, Store};

/// Basic CRUD-ish views using server/src/api_user.rs
#[function_component(UsersComponent)]
pub(crate) fn users_component() -> Html {
    // let users_state = use_state(|| None);
    let (store, _dispatch) = use_store::<PersistentStore>();
    let (_store, dispatch) = use_store::<Store>();

    let token = store.token.clone().unwrap_or_default();

    // Use the use_async hook to manage the state of the async operation
    // cf https://github.com/jetli/yew-hooks/blob/4bfd4393dae5b1a600cfdb58ea05649ce2e30da5/examples/yew-app/src/app/hooks/use_async.rs#L34
    let users_response_state = use_async_with_options(
        async move { api_list_users(&token).await },
        UseAsyncOptions::enable_auto(),
    );

    // Render the table
    html! {
        <div class="flex flex-col min-h-screen flex-grow items-center space-x-4">
        <table class="table-auto">
            <thead>
                <tr>
                    <th>{"Username"}</th>
                    <th>{"Superuser?"}</th>
                </tr>
            </thead>
            <tbody>
                {
                    if let Some(users) = &users_response_state.data {
                        users.users.iter().map(|user| html! {
                            <tr>
                                <td>{&user.username}</td>
                                <td>{&user.is_super_user}</td>
                            </tr>
                        }).collect::<Html>()
                    }
                    else if let Some(error) = &users_response_state.error {
                        console::error_1(&format!("api_list_users error: {error:?}",).into());
                        set_page_loading(false, &dispatch);
                        set_show_alert(error.to_string(), &dispatch);
                        html! { format!("Error: {}", error) }
                    }
                    else if users_response_state.loading {
                        console::log_1(&format!("api_list_users loading...").into());
                        html! { "Loading..." }
                    }
                    else {
                        html! {}
                    }
                }
            </tbody>
        </table>
        </div>
    }
}
