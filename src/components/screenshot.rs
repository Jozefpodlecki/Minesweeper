use yew::*;
use log::*;

use crate::{services::{ScreenshotService, ToastManager}, utils::timestamped_filename};
use yew_icons::{Icon, IconData};

#[function_component(Screenshot)]
pub fn screenshot() -> Html {
    let toast_manager = unsafe { use_context::<ToastManager>().unwrap_unchecked() };
    let screenshot_service = unsafe { use_context::<ScreenshotService>().unwrap_unchecked() };

    let on_screenshot: Callback<MouseEvent> = {
        Callback::from(move |_| {
            let toast_manager = toast_manager.clone();
            let screenshot_service = screenshot_service.clone();
            
            wasm_bindgen_futures::spawn_local(async move {
                let capture = match screenshot_service.capture_body().await {
                    Ok(value) => value,
                    Err(err) => {
                        toast_manager.send(err);
                         return
                    },
                };

                let file_name = timestamped_filename();
                screenshot_service.download(capture, &file_name);
                toast_manager.success("Downloaded screenshot");
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