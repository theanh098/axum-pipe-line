use serde::Deserialize;

#[derive(Deserialize)]
pub struct NFTFetched {
  name: Option<String>,
  image_url: String,
  permalink: String,
  description: Option<String>,
  image_thumbnail_url: Option<String>,
  collection: CollectionFetched,
}

#[derive(Deserialize)]
pub struct CollectionFetched {
  slug: String,
  name: String,
  description: Option<String>,
  image_url: Option<String>,
}
