use web_sys::console;
/// `https://github.com/wpcodevo/rust-yew-signup-signin/blob/62e9186ba1ede01b6d13eeeac036bbd56a131e1e/src/router.rs`
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
    users_component::UsersComponent,
};

#[derive(Clone, Routable, PartialEq, Debug)]
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
    #[not_found]
    #[at("/404")]
    NotFound,
    #[at("/users")]
    UsersComponent,
}

#[allow(clippy::needless_pass_by_value)]
pub fn switch(routes: Route) -> Html {
    match routes {
        Route::HomePage => html! {<HomePage/> },
        // Route::RegisterPage => html! {<RegisterPage/> },
        Route::LoginPage => html! {<LoginPage/> },
        // Route::ProfilePage => html! {<ProfilePage/> },
        Route::NotFound => {
            console::log_1(&format!("Route not found: {routes:?}",).into());
            html! { <My404Page /> }
        }
        Route::UsersComponent => html! {<UsersComponent/> },
    }
}

#[function_component(My404Page)]
pub fn not_found() -> Html {
    // https://merakiui.com/components/marketing/404-pages
    html! {
       <section class="bg-white dark:bg-gray-900 ">
       <div class="container flex items-center min-h-screen px-6 py-12 mx-auto">
           <div>
               <p class="text-sm font-medium text-blue-500 dark:text-blue-400">{"404 error"}</p>
               <h1 class="mt-3 text-2xl font-semibold text-gray-800 dark:text-white md:text-3xl">{"We can’t find that page"}</h1>
               <p class="mt-4 text-gray-500 dark:text-gray-400">{"Sorry, the page you are looking for doesn't exist or has been moved."}</p>

               <div class="flex items-center mt-6 gap-x-3">
                   <button class="flex items-center justify-center w-1/2 px-5 py-2 text-sm text-gray-700 transition-colors duration-200 bg-white border rounded-lg gap-x-2 sm:w-auto dark:hover:bg-gray-800 dark:bg-gray-900 hover:bg-gray-100 dark:text-gray-200 dark:border-gray-700">
                       <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-5 h-5 rtl:rotate-180">
                           <path stroke-linecap="round" stroke-linejoin="round" d="M6.75 15.75L3 12m0 0l3.75-3.75M3 12h18" />
                       </svg>

                       <span>{"Go back"}</span>
                   </button>

                   <button class="w-1/2 px-5 py-2 text-sm tracking-wide text-white transition-colors duration-200 bg-blue-500 rounded-lg shrink-0 sm:w-auto hover:bg-blue-600 dark:hover:bg-blue-500 dark:bg-blue-600">
                       {"Take me home"}
                   </button>
               </div>
           </div>
       </div>
       </section>
    }
}
