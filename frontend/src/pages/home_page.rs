use std::rc::Rc;

use yew::prelude::*;

use crate::components::header::Header;
use crate::pages::geo_loc_component::GeoLocComponent;
use crate::pages::map_component::MapComponent;
use crate::pages::websocket_chat_component::WebSocketChatComponent;
use crate::pages::websocket_geoloc_component::WebSocketGeoLocComponent;

/// https://github.com/yewstack/yew/blob/d0419a278dc126af4556c9afae2ef6b00b5fef36/examples/contexts/src/msg_ctx.rs#L5
#[derive(Clone, Debug, PartialEq)]
pub struct AppCtxInternal {
    pub(crate) username: Option<String>,
}

impl Reducible for AppCtxInternal {
    type Action = String;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        AppCtxInternal {
            username: Some(action),
        }
        .into()
    }
}

pub type AppCtx = UseReducerHandle<AppCtxInternal>;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    // TODO? but does not work with "struct Component", only function_component
    let app_ctx = use_reducer(|| AppCtxInternal { username: None });

    html! {
      <>
        <Header />

        <section class="bg-ct-blue-600 min-h-screen pt-20">
            <div class="max-w-4xl mx-auto bg-ct-dark-100 rounded-md h-[20rem] flex justify-center items-center">
                <p class="text-3xl font-semibold">{"Welcome to Rust, Yew.rs and WebAssembly"}</p>
            </div>
        </section>

        <ContextProvider<AppCtx> context={app_ctx}>

            // <Switch<Route> render={switch} />
            <MapComponent markers={vec![]}/>
            <WebSocketChatComponent />
            <GeoLocComponent />
            <WebSocketGeoLocComponent />
            // <UsernameForm username=""/>
        </ContextProvider<AppCtx>>

        </>
    }
}
