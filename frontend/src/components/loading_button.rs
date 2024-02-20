/// https://github.com/wpcodevo/rust-yew-signup-signin/blob/62e9186ba1ede01b6d13eeeac036bbd56a131e1e/src/components/loading_button.rs
///
use super::spinner::Spinner;
use yew::prelude::*;

#[derive(Debug, Properties, PartialEq)]
pub struct Props {
    pub loading: bool,
    #[prop_or_default]
    pub btn_color: Option<String>,
    #[prop_or_default]
    pub text_color: Option<String>,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(LoadingButton)]
pub fn loading_button_component(props: &Props) -> Html {
    let text_color = props
        .text_color
        .clone()
        .unwrap_or_else(|| "text-white".to_string());
    let btn_color = props
        .btn_color
        .clone()
        .unwrap_or_else(|| "bg-ct-yellow-600".to_string());

    html! {
    <button
      type="submit"
      class={format!(
        "w-full py-3 font-semibold rounded-lg outline-none border-none flex justify-center {}",
         if props.loading {"bg-[#ccc]"} else {btn_color.as_str()}
      )}
    >
      if props.loading {
        <div class="flex items-center gap-3">
          <Spinner />
          <span class="text-slate-500 inline-block">{"Loading..."}</span>
        </div>
      }else{
        <span class={text_color.to_owned()}>{props.children.clone()}</span>
      }
    </button>
    }
}
