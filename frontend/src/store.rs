use std::collections::HashMap;

/// https://github.com/wpcodevo/rust-yew-signup-signin/blob/62e9186ba1ede01b6d13eeeac036bbd56a131e1e/src/store.rs
///
use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

use crate::api::types::User;

#[derive(Debug, PartialEq, Serialize, Deserialize, Default, Clone)]
pub struct AlertInput {
    pub show_alert: bool,
    pub alert_message: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, Store)]
pub struct Store {
    pub page_loading: bool,
    pub alert_input: AlertInput,
    pub locations: HashMap<String, (f64, f64)>,
}

/// We split the "Store" in two: a part that is in memory only; and this: that is persisted with local storage (cookies)
#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, Store)]
#[store(storage = "local")]
pub struct PersistentStore {
    pub auth_user: Option<User>,
    pub token: Option<String>,
}

pub fn set_page_loading(loading: bool, dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.page_loading = loading;
    })
}

pub fn set_auth_user(
    user: Option<User>,
    token: Option<String>,
    dispatch: Dispatch<PersistentStore>,
) {
    dispatch.reduce_mut(move |store| {
        store.auth_user = user;
        store.token = token;
    })
}

pub fn set_show_alert(message: String, dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.alert_input = AlertInput {
            alert_message: message,
            show_alert: true,
        };
    })
}

pub fn set_hide_alert(dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.alert_input.show_alert = false;
    })
}
