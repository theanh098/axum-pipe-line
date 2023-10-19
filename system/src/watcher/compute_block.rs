use futures::future;
use image::io::Reader as ImageReader;
use image::{GenericImage, ImageBuffer};
use sea_orm::DatabaseConnection;
use server::errors::AppError;
use server::shared::database::repositories::NftRepository;
use std::io::Cursor;

type ImageBuf = Vec<u8>;

async fn img_url_to_buffer(url: impl Into<String>) -> Result<ImageBuf, surf::Error> {
  let mut response = surf::get(url.into()).await?;

  let img_bytes = response.body_bytes().await?;

  let dynamic_image = ImageReader::new(Cursor::new(img_bytes))
    .with_guessed_format()?
    .decode()?;

  let mut bytes: ImageBuf = Vec::new();

  dynamic_image.write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png)?;

  Ok(bytes)
}

fn get_position_range_on_block(block: u32) -> (u32, u32) {
  let start_position = 1 + (1..block).map(|i| i.pow(2)).sum::<u32>();
  let end_position = (1..=block).map(|i| i.pow(2)).sum::<u32>();
  (start_position, end_position)
}

async fn generate_block(block: u32, db_conn: &DatabaseConnection) -> Result<(), AppError> {
  const BLOCK_SIZE: u32 = 80;
  let (start, end) = get_position_range_on_block(block);

  let mut image_block = ImageBuffer::new(BLOCK_SIZE, BLOCK_SIZE);
  let child_image_size: u32 = BLOCK_SIZE / block;

  future::join_all(
    NftRepository::new(&db_conn)
      .find_active_nfts_with_position_range(start, end)
      .await
      .map(|nfts| {
        nfts
          .into_iter()
          .map(|nft| tokio::spawn(async { img_url_to_buffer(nft.image_url).await.unwrap() }))
      })?,
  )
  .await
  .into_iter()
  .enumerate()
  .for_each(|(idx, join_result)| {
    let _ = join_result.map(|buffer| {
      let child_image = image::load_from_memory(&buffer).unwrap().resize_exact(
        child_image_size as u32,
        child_image_size as u32,
        image::imageops::FilterType::Lanczos3,
      );

      let x: u32 = ((idx as u32) % block) * (BLOCK_SIZE / block);
      let y: u32 = ((idx as u32) / block) * (BLOCK_SIZE / block);

      let _ = image_block.copy_from(&child_image, x, y);
    });
  });

  image_block
    .save(format!(
      "{}/images/block_{block}.png",
      std::env::current_dir().unwrap().to_str().unwrap()
    ))
    .map_err(|err| err.into())
}

pub async fn generate_9_block_images(db_conn: &DatabaseConnection) {
  future::join_all((1..=9).into_iter().map(|block| {
    let db_conn = db_conn.to_owned();
    tokio::spawn(async move { generate_block(block, &db_conn).await.unwrap() })
  }))
  .await;
}
