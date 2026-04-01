use js_sys::Promise;
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{File, FileReader, ProgressEvent};

use crate::models::AppError;

pub struct AsyncFileReader;

impl AsyncFileReader {
    pub async fn read_as_data_url(file: File) -> Result<String, AppError> {
        let reader = FileReader::new()?;

        let promise = Promise::new(&mut |resolve, reject| {
            let reader_clone = reader.clone();

            let onload = {
                let reject = reject.clone();
                Closure::<dyn FnMut(ProgressEvent)>::once(move |_| {
                    match reader_clone.result() {
                        Ok(result) => {
                            resolve.call1(&JsValue::NULL, &result).unwrap();
                        }
                        Err(err) => {
                            reject.call1(&JsValue::NULL, &err).unwrap();
                        }
                    }
                })
            };

            let onerror = Closure::<dyn FnMut(ProgressEvent)>::once(move |_| {
                reject.call0(&JsValue::NULL).unwrap();
            });

            reader.set_onload(Some(onload.as_ref().unchecked_ref()));
            reader.set_onerror(Some(onerror.as_ref().unchecked_ref()));

            onload.forget();
            onerror.forget();
        });

        reader.read_as_data_url(&file)?;

        let result = JsFuture::from(promise).await?;
        result
            .as_string()
            .ok_or_else(|| AppError::failed_to_read_body("Expected Data URL string".into()))
    }
}