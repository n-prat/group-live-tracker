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
    <div class="flex flex-col min-h-screen flex-grow">
        <header class="bg-blue-500 p-4 text-black">
            <Header />
        </header>

        <div class="flex flex-row flex-grow">
            <div class="basis-3/4 bg-gray-200 p-4">
                <MapComponent markers={vec![(PARIS_LAT, PARIS_LNG)]}/>
            </div>

            <div class="basis-1/4 bg-gray-200 p-4">
                <WebSocketChatComponent />
            </div>

            <GeoLocComponent />
            <WebSocketGeoLocComponent />
        </div>


        // <ContextProvider<AppCtx> context={app_ctx}>
        // <Switch<Route> render={switch} />
        // </ContextProvider<AppCtx>>

    </div>
    }
}
