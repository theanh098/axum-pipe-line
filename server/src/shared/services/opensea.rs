use std::time::Duration;

use serde::de::DeserializeOwned;

pub struct OpenseaService(surf::Client);

impl OpenseaService {
    pub fn new() -> Self {
        let opensea_api_key =
            std::env::var("OPENSEA_API_KEY").expect("opensea api key must be set");

        surf::Config::new()
            .set_base_url(surf::Url::parse("https://api.opensea.io/api/v1/").unwrap())
            .add_header("X-API-KEY", opensea_api_key)
            .unwrap()
            .set_timeout(Some(Duration::from_secs(5)))
            .try_into()
            .map(|client| OpenseaService(client))
            .unwrap()
    }

    pub async fn fetch_nft<T: DeserializeOwned>(
        &self,
        token_address: &str,
        token_id: &str,
    ) -> Result<T, surf::Error> {
        self.0
            .get(format!("asset/{token_address}/{token_id}"))
            .await
            .map(|mut res| async move { res.body_json::<T>().await })?
            .await
    }
}
