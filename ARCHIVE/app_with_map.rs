use crate::chat::chat_model::ChatModel;
use crate::chat::web_rtc_manager::WebRTCManager;
use crate::map_component::MapComponent;
use yew::prelude::*; // Replace `your_chat_model` with the actual module name

pub struct AppWithMap {}

impl Component for AppWithMap {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        AppWithMap {}
    }

    fn update(&mut self, ctx: &Context<Self>, _: Self::Message) -> bool {
        true
    }

    // fn change(&mut self, _: Self::Properties) -> bool {
    //     false
    // }

    fn view(&self, ctx: &Context<Self>) -> Html {
        log::warn!("view");

        html! {
            <>
                <ChatModel<WebRTCManager> />
                <MapComponent />
            </>
        }
    }
}
