use std::{ops::Deref, pin::Pin};

use log::*;

use crate::{blob::ImageBlob, models::{AppError, Social}, services::HttpClient};

#[derive(Clone)]
pub struct ApiClient(pub Box<dyn ApiClientTrait>);

impl PartialEq for ApiClient {
    fn eq(&self, _: &Self) -> bool {
        false
    }
}

impl Deref for ApiClient {
    type Target = dyn ApiClientTrait;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

pub trait ApiClientTrait: Send + Sync {
    fn clone_box(&self) -> Box<dyn ApiClientTrait>;
    fn get_social(&self) -> Pin<Box<dyn Future<Output = Result<Social, AppError>>>>;
    fn get_image(&self, url: &str) -> Pin<Box<dyn Future<Output = Result<ImageBlob, AppError>>>>;
}

impl Clone for Box<dyn ApiClientTrait> {
    fn clone(&self) -> Box<dyn ApiClientTrait> {
        self.clone_box()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LiveApiClient(HttpClient);

impl ApiClientTrait for LiveApiClient {
    fn clone_box(&self) -> Box<dyn ApiClientTrait> {
        Box::new(self.clone())
    }

    fn get_social(&self) -> Pin<Box<dyn Future<Output = Result<Social, AppError>>>> {
        let http = self.0.clone();

        Box::pin(async move {
            let url = "public/social.json";
            let data = http.get_as_json(url).await?;
            Ok(data)
        })
    }

    fn get_image(&self, url: &str) -> Pin<Box<dyn Future<Output = Result<ImageBlob, AppError>>>> {
        let http = self.0.clone();
        let url = url.to_string();
        Box::pin(async move {
            let blob = http.get_as_blob(&url).await?;
            let image = ImageBlob::try_from(blob).await?;
            Ok(image)
        })
    }
}

impl LiveApiClient {
    pub fn new(http_client: HttpClient) -> Self {
        Self(http_client)
    }
}