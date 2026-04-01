use js_sys::Promise;
use log::debug;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{window, Blob, FileReader, Headers, HtmlImageElement, Request, RequestCache, RequestInit, RequestMode, Response, Url, Window};

use crate::{models::{AppError, Social}, services::HttpClient};

#[cfg(debug_assertions)]
const CACHE_MODE: RequestCache = RequestCache::NoStore;

#[cfg(not(debug_assertions))]
const CACHE_MODE: RequestCache = RequestCache::Default;

#[derive(Debug, Clone, PartialEq)]
pub struct ApiClient(HttpClient);

impl ApiClient {
    pub fn new(http_client: HttpClient) -> Self {
        Self(http_client)
    }

    pub async fn blob_to_data_url(blob: &Blob) -> Result<String, AppError> {
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

        file_reader.read_as_data_url(&blob)?;

        let js_value = JsFuture::from(promise).await?;
        Ok(js_value.as_string().unwrap())
    }

    pub async fn validate_image_blob(blob: &Blob) -> Result<(), AppError> {
        let url = Url::create_object_url_with_blob(blob)?;

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

        Ok(())
    }

    pub async fn get_image(&self, url: &str) -> Result<Blob, AppError> {
        let blob = self.0.get_as_blob(url).await?;

        Self::validate_image_blob(&blob).await?;

        Ok(blob)
    }

    pub async fn get_social(&self) -> Result<Social, AppError> {
        let url = "public/social.json";
        let data = self.0.get_as_json(url).await?;

        Ok(data)
    }
}