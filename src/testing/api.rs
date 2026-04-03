use std::pin::Pin;

use crate::{api::{ApiClientTrait}, blob::ImageBlob, models::{AppError, Social}, services::HttpClient};

#[cfg(debug_assertions)]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct E2EConfig {
    pub fail_on_get_social_fetch: bool
}

#[derive(Debug, Clone, PartialEq)]
pub struct MockApiClient(HttpClient, E2EConfig);

impl ApiClientTrait for MockApiClient {
    fn clone_box(&self) -> Box<dyn ApiClientTrait> {
        Box::new(self.clone())
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

    fn get_social(&self) -> Pin<Box<dyn Future<Output = Result<Social, AppError>>>> {
        let http = self.0.clone();
        let fail_on_get_social_fetch = self.1.fail_on_get_social_fetch;

        Box::pin(async move {

            if fail_on_get_social_fetch {
                return Err(AppError::network_request_failed("e2e".into()));
            }

            let url = "public/social.json";
            let data = http.get_as_json(url).await?;
            Ok(data)
        })
    }
}

impl MockApiClient {
    pub fn new(http_client: HttpClient, config: E2EConfig) -> Self {
        Self(http_client, config)
    }
}