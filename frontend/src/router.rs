/// https://github.com/wpcodevo/rust-yew-signup-signin/blob/62e9186ba1ede01b6d13eeeac036bbd56a131e1e/src/router.rs
///
use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{
    home_page::HomePage,
    login_page::LoginPage,
    // TODO
    // profile_page::ProfilePage,
    // TODO
    // register_page::RegisterPage,
};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    HomePage,
    // TODO
    // #[at("/register")]
    // RegisterPage,
    #[at("/login")]
    LoginPage,
    // TODO
    // #[at("/profile")]
    // ProfilePage,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::HomePage => html! {<HomePage/> },
        // Route::RegisterPage => html! {<RegisterPage/> },
        Route::LoginPage => html! {<LoginPage/> },
        // Route::ProfilePage => html! {<ProfilePage/> },
    }
}
