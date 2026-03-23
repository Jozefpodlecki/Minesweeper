use chrono::Utc;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{window, HtmlAnchorElement, HtmlCanvasElement, HtmlElement};
use yew::*;
use log::*;

use crate::{ffi::html2canvas, models::AppError, route::Route, services::ToastManager};
use yew_icons::{Icon, IconData};


pub async fn take_screnshot(element: HtmlElement) -> Result<(), AppError> {
    let promise = html2canvas(element.into())?;
    let canvas = JsFuture::from(promise).await?;
    let canvas: HtmlCanvasElement = canvas.unchecked_into();
    let data_url = canvas.to_data_url_with_type_and_encoder_options("image/jpeg", &JsValue::from_f64(0.9))?;

    let navigator = window().unwrap().navigator();
    let clipboard = navigator.clipboard();
    JsFuture::from(clipboard.write_text(&data_url)).await?;

    let timestamp = Utc::now().format("%y%m%d%H%M%S").to_string();
    let file_name = format!("{}.jpg", timestamp);
    trigger_download(&data_url, &file_name);

    Ok(())
}

pub fn trigger_download(data_url: &str, filename: &str) {
    unsafe {
        let document = window()
            .unwrap_unchecked()
            .document()
            .unwrap_unchecked();

        let anchor: HtmlAnchorElement = document
            .create_element("a")
            .unwrap_unchecked()
            .unchecked_into();

        anchor.set_href(data_url);
        anchor.set_download(filename);

        let anchor_style = anchor.style();
        anchor_style.set_property("display", "none").unwrap_unchecked();

        let body: HtmlElement = document.body().unwrap_unchecked();

        body.append_child(&anchor).unwrap_unchecked();
        anchor.click();
        body.remove_child(&anchor).unwrap_unchecked();
    }
}

#[function_component(Screenshot)]
pub fn screenshot() -> Html {
    let toast_manager = unsafe { use_context::<ToastManager>().unwrap_unchecked() };

    let on_screenshot: Callback<MouseEvent> = {
        Callback::from(move |event| {
            let toast_manager = toast_manager.clone();
            let window = window().expect("Windows object not found");
            let document = window.document().expect("Document object not found");
            let body: HtmlElement = document.body().expect("Document body not found");
            
            wasm_bindgen_futures::spawn_local(async move {
                if let Err(err) = take_screnshot(body).await {
                    toast_manager.send(err);
                }
                
            });
        })
    };

    html! {
        <>
            <section class="absolute top-0 right-0 text-white">
                <button
                    type="button"
                    onclick={on_screenshot}
                    class="p-2 hover:bg-white/10 rounded transition"
                >
                    <Icon data={IconData::LUCIDE_CAMERA} width={"20px"} />
                </button>
            </section>
        </>
    }

}