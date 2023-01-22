use wasm_bindgen::JsCast;
use web_sys::HtmlDocument;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct CopyToClipboardProps {
    pub value: AttrValue,
    #[prop_or_default]
    pub input_id: AttrValue,
    #[prop_or_default]
    pub button_id: AttrValue,
}

#[function_component(CopyToClipboard)]
pub fn copy_to_clipboard(props: &CopyToClipboardProps) -> Html {
    let input_ref = use_node_ref();
    let onclick = {
        let input_ref = input_ref.clone();
        Callback::from(move |_| {
            let input = input_ref
                .cast::<HtmlInputElement>()
                .expect("Could not get input element of CopyToClipboard component");
            input.select();
            let document = web_sys::window()
                .expect("Could not get window")
                .document()
                .expect("Could not get document")
                .dyn_into::<HtmlDocument>()
                .expect("Could not cast document to HtmlDocument");
            // FIXME: Use clipboard API once stabilized in web_sys
            let result = document.exec_command("copy");
            log::info!(
                "Copied to clipboard with return value '{}'",
                result.unwrap_or(false)
            );
        })
    };

    html! {
       <>
            <input
                type={"text"}
                id={props.input_id.clone()}
                ref={input_ref}
                value={props.value.clone()}
                readonly={true}
            />
            <button
                id={props.button_id.clone()}
                onclick={onclick}
                style="margin-left: 4px"
             >
                {"Copy"}
            </button>
       </>
    }
}
