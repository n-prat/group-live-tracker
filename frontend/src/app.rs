/// https://github.com/wpcodevo/rust-yew-signup-signin/blob/62e9186ba1ede01b6d13eeeac036bbd56a131e1e/src/app.rs
///
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::use_store;

use crate::components::{
    alert::{AlertComponent, Props as AlertProps},
    spinner::Spinner,
};
use crate::router::{switch, Route};
use crate::store::Store;

// TODO FIXME this is an awful way to handle PROD vs LOCAL dev
#[cfg(debug_assertions)]
pub(crate) const WS_ROOT: &str = "wss://localhost:8081/ws";
#[cfg(debug_assertions)]
pub(crate) const API_ROOT: &str = "https://localhost:8081";
#[cfg(not(debug_assertions))]
pub(crate) const WS_ROOT: &str = "wss://tracker.nathanprat.fr/ws";
#[cfg(not(debug_assertions))]
pub(crate) const API_ROOT: &str = "https://tracker.nathanprat.fr";

#[function_component(App)]
pub fn app() -> Html {
    let (store, _) = use_store::<Store>();
    let message = store.alert_input.alert_message.clone();
    let show_alert = store.alert_input.show_alert;
    let is_page_loading = store.page_loading.clone();

    let alert_props = AlertProps {
        message,
        delay_ms: 5000,
    };
    html! {
        <BrowserRouter>
                <Switch<Route> render={switch} />
                if show_alert {
                    <AlertComponent
                        message={alert_props.message}
                        delay_ms={alert_props.delay_ms}
                     />
                }
                if is_page_loading {
                    <div class="pt-4 pl-2 top-[5.5rem] fixed">
                        <Spinner width={Some("1.5rem")} height={Some("1.5rem")} color="text-ct-yellow-600" />
                    </div>
                }
        </BrowserRouter>
    }
}
