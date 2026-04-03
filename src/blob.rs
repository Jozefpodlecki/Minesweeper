use js_sys::Promise;
use log::debug;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Blob, FileReader, HtmlImageElement, Url};

use crate::models::AppError;

#[derive(Clone, Debug)]
pub struct ImageBlob(Blob);

impl ImageBlob {
    pub async fn to_data_url(self) -> Result<String, AppError> {
        let file_reader = FileReader::new()?;

        let promise = Promise::new(&mut |resolve, reject| {
            let fr_clone = file_reader.clone();
            let resolve = resolve.clone();
            let reject = reject.clone();

            let onload = Closure::once_into_js(move || {
                let result = fr_clone.result().unwrap();
                resolve.call1(&JsValue::NULL, &result).unwrap();
            });

            let onerror = Closure::once_into_js(move || {
                reject.call0(&JsValue::NULL).unwrap();
            });

            file_reader.set_onload(Some(onload.as_ref().unchecked_ref()));
            file_reader.set_onerror(Some(onerror.as_ref().unchecked_ref()));
        });

        file_reader.read_as_data_url(&self.0)?;

        let js_value = JsFuture::from(promise).await?;
        Ok(js_value.as_string().unwrap())
    }

    pub async fn try_from(blob: Blob) -> Result<Self, AppError> {
        let url = Url::create_object_url_with_blob(&blob)?;

        let img = HtmlImageElement::new()?;

        let promise = Promise::new(&mut |resolve, reject| {
            let resolve = resolve.clone();
            let reject = reject.clone();

            let onload = Closure::once_into_js(move || {
                resolve.call0(&JsValue::NULL).unwrap();
            });

            let onerror = Closure::once_into_js(move || {
                reject.call0(&JsValue::NULL).unwrap();
            });

            img.set_onload(Some(onload.as_ref().unchecked_ref()));
            img.set_onerror(Some(onerror.as_ref().unchecked_ref()));
        });

        img.set_src(&url);

        let result = JsFuture::from(promise).await;

        Url::revoke_object_url(&url)?;

        result.map_err(AppError::invalid_file_format)?;

        Ok(Self(blob))
    }

    pub fn as_blob(&self) -> &Blob {
        &self.0
    }
}