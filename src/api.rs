use log::debug;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{window, Headers, Request, RequestCache, RequestInit, RequestMode, Response, Window};

use crate::models::{AppError, Social};

#[cfg(debug_assertions)]
const CACHE_MODE: RequestCache = RequestCache::NoStore;

#[cfg(not(debug_assertions))]
const CACHE_MODE: RequestCache = RequestCache::Default;

#[derive(Debug, Clone, PartialEq)]
pub struct ApiClient(Window);

impl ApiClient {
    pub fn new(window: Window) -> Self {
        Self(window)
    }

    pub async fn get_social(&self) -> Result<Social, AppError> {
        let url = "public/social.json";

        let request_options = RequestInit::new();
        request_options.set_method("GET");
        request_options.set_mode(RequestMode::Cors);
        request_options.set_cache(CACHE_MODE);

        let request = Request::new_with_str_and_init(url, &request_options)
            .map_err(AppError::failed_to_build_request)?;

        let response_value = JsFuture::from(self.0.fetch_with_request(&request))
            .await
            .map_err(AppError::network_request_failed)?;

        let response: Response = response_value.dyn_into()
            .map_err(AppError::invalid_response)?;

        let js_value = JsFuture::from(response.json()?)
            .await
            .map_err(AppError::failed_to_read_body)?;

        let data: Social = serde_wasm_bindgen::from_value(js_value)
            .map_err(AppError::from)?;

        Ok(data)
    }
}