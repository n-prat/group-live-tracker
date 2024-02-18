use std::rc::Rc;

use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::console;
use web_sys::MessageEvent;
use web_sys::WebSocket;
use yew::Callback;
use yew::Component;
use yew::Context;

use crate::AppCtx;

pub(crate) fn new_websocket(
    protocol: &str,
    on_message_callback: Callback<MessageEvent, ()>,
    on_open_callback: Callback<MessageEvent, ()>,
    on_close_callback: Callback<MessageEvent, ()>,
    on_error_callback: Callback<MessageEvent, ()>,
) -> WebSocket {
    let protocols = js_sys::Array::new();
    protocols.push(&JsValue::from(protocol));
    // let ws = WebSocket::new("ws://localhost:8081/ws")
    //     .expect(" WebSocket::new_with_str_sequence failed!");
    let ws = WebSocket::new_with_str_sequence("ws://localhost:8081/ws", &protocols)
        .expect(" WebSocket::new_with_str_sequence failed!");

    let on_message_callback_rc = Rc::new(on_message_callback);
    let on_message_closure = Closure::wrap(Box::new(move |event: MessageEvent| {
        let callback = on_message_callback_rc.clone();
        callback.emit(event);
        // if let Err(err) = result {
        //     log::error!("Failed to process WebSocket message event: {:?}", err);
        // }
    }) as Box<dyn FnMut(MessageEvent)>);

    ws.clone()
        .set_onmessage(Some(on_message_closure.as_ref().unchecked_ref()));
    on_message_closure.forget(); // Ensure closure is not dropped prematurely

    // MUST wait for the cx to be ready ???
    // ws.send_with_str("hello from rendered")
    //     .expect("rendered send_with_str failed");

    let on_open_callback_rc = Rc::new(on_open_callback);
    let on_open_closure: Closure<dyn FnMut(MessageEvent)> =
        Closure::wrap(Box::new(move |event: MessageEvent| {
            let callback = on_open_callback_rc.clone();
            callback.emit(event);
            // if let Err(err) = result {
            //     log::error!("Failed to process WebSocket message event: {:?}", err);
            // }
        }) as Box<dyn FnMut(MessageEvent)>);
    ws.clone()
        .set_onopen(Some(on_open_closure.as_ref().unchecked_ref()));
    on_open_closure.forget();

    let on_close_callback_rc = Rc::new(on_close_callback);
    let on_close_closure: Closure<dyn FnMut(MessageEvent)> =
        Closure::wrap(Box::new(move |event: MessageEvent| {
            let callback = on_close_callback_rc.clone();
            callback.emit(event);
        }) as Box<dyn FnMut(MessageEvent)>);
    ws.clone()
        .set_onclose(Some(on_close_closure.as_ref().unchecked_ref()));
    on_close_closure.forget();

    let on_error_callback_rc = Rc::new(on_error_callback);
    let on_error_closure: Closure<dyn FnMut(MessageEvent)> =
        Closure::wrap(Box::new(move |event: MessageEvent| {
            let callback = on_error_callback_rc.clone();
            callback.emit(event);
        }) as Box<dyn FnMut(MessageEvent)>);
    ws.clone()
        .set_onerror(Some(on_error_closure.as_ref().unchecked_ref()));
    on_error_closure.forget();

    ws
}

pub(crate) fn get_username_from_context<T: Component>(ctx: &Context<T>) -> Option<String> {
    // let app_ctx: AppCtx = use_context();
    let (app_ctx, _) = ctx
        .link()
        .context::<AppCtx>(Callback::noop())
        .expect("No AppCtx Provided");

    app_ctx.username.clone()
}

pub(crate) fn update_username_in_context<T: Component>(ctx: &Context<T>, username: &str) {
    // let app_ctx: AppCtx = use_context();
    let (app_ctx, _ctx_handle) = ctx
        .link()
        .context::<AppCtx>(Callback::noop())
        .expect("No AppCtx Provided");

    // app_ctx.dispatch("aaa");
    // app_ctx.borrow_mut().username = Some(username.to_string());
    // app_ctx.username = Some("AAAAAAAAA".to_string());
    app_ctx.dispatch(username.to_string());

    console::log_1(&format!("update_username_in_context: done: {:?}", username).into());
}
