use yew::prelude::*;
use yewdux::prelude::*;

use crate::components::header::Header;
use crate::pages::geo_loc_component::GeoLocComponent;
use crate::pages::login_page::LoginPage;
use crate::pages::map_component::{MapComponent, PARIS_LAT, PARIS_LNG};
use crate::pages::websocket_chat_component::WebSocketChatComponent;
use crate::pages::websocket_geoloc_component::WebSocketGeoLocComponent;
use crate::store::PersistentStore;

// /// https://github.com/yewstack/yew/blob/d0419a278dc126af4556c9afae2ef6b00b5fef36/examples/contexts/src/msg_ctx.rs#L5
// #[derive(Clone, Debug, PartialEq)]
// pub struct AppCtxInternal {
//     pub(crate) username: Option<String>,
// }

// impl Reducible for AppCtxInternal {
//     type Action = String;

//     fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
//         AppCtxInternal {
//             username: Some(action),
//         }
//         .into()
//     }
// }

// pub type AppCtx = UseReducerHandle<AppCtxInternal>;

#[function_component(HomePage)]
pub(crate) fn home_page() -> Html {
    // TODO(auth) move that into a custom hook???
    // MAYBE see https://yew.rs/docs/next/concepts/suspense ?
    // and maybe https://github.com/yewstack/yew/issues/1526
    // This is way more contrived than it should be...
    let (store, _dispatch) = use_store::<PersistentStore>();
    let auth_user = store.auth_user.clone();

    if auth_user.is_none() {
        return html! {<LoginPage />};
    }

    html! {
      <>
        <Header />

        <section class="bg-ct-blue-600 min-h-screen pt-20">
            <div class="max-w-4xl mx-auto bg-ct-dark-100 rounded-md h-[20rem] flex justify-center items-center">
                <p class="text-3xl font-semibold">{"Welcome to Rust, Yew.rs and WebAssembly"}</p>
            </div>
        </section>

        // <ContextProvider<AppCtx> context={app_ctx}>

        // <Switch<Route> render={switch} />
        <MapComponent markers={vec![(PARIS_LAT, PARIS_LNG)]}/>
        <WebSocketChatComponent />
        <GeoLocComponent />
        <WebSocketGeoLocComponent />
        // <UsernameForm username=""/>

        // </ContextProvider<AppCtx>>

        </>
    }
}
