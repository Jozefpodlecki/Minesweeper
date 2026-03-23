use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Document, HtmlAnchorElement, HtmlCanvasElement, HtmlElement, Navigator};
use log::*;

use crate::{ffi::html2canvas, models::AppError};

pub struct ScreenshotOutput(String);

impl ScreenshotOutput {
    pub fn to_data_url(self) -> String {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ScreenshotService {
    document: Document,
    body: HtmlElement,
    navigator: Navigator
}

impl ScreenshotService {
    pub fn new(document: Document, body: HtmlElement, navigator: Navigator) -> Self {
        Self {
            document,
            body,
            navigator
        }
    }

    pub async fn capture_body(&self) -> Result<ScreenshotOutput, AppError> {
        self.capture(&self.body).await
    }

    pub async fn capture(&self, element: &HtmlElement) -> Result<ScreenshotOutput, AppError> {
        debug!("Capturing screenshot of element");

        let promise = html2canvas(element.into())?;
        let canvas = JsFuture::from(promise).await?;
        let canvas: HtmlCanvasElement = canvas.unchecked_into();

        let data_url = canvas.to_data_url_with_type_and_encoder_options(
            "image/jpeg",
            &JsValue::from_f64(0.9),
        )?;

        Ok(ScreenshotOutput(data_url))
    }

    pub async fn copy_to_clipboard(&self, data_url: &str) -> Result<(), AppError> {
        debug!("Copying to cliboard");
        let clipboard = self.navigator.clipboard();

        JsFuture::from(clipboard.write_text(data_url)).await?;

        Ok(())
    }

    pub fn download(&self, output: ScreenshotOutput, file_name: &str) {
        unsafe {
            debug!("Triggering download: {}", file_name);
            
            let anchor: HtmlAnchorElement = self.document
                .create_element("a")
                .unwrap_unchecked()
                .unchecked_into();

            anchor.set_href(&output.to_data_url());
            anchor.set_download(file_name);

            let anchor_style = anchor.style();
            anchor_style.set_property("display", "none").unwrap_unchecked();

            self.body.append_child(&anchor).unwrap_unchecked();
            anchor.click();
            self.body.remove_child(&anchor).unwrap_unchecked();
        }
    }
}