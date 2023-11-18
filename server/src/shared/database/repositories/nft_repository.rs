use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};

use crate::shared::database::entities::{nft, prelude::Nft};

pub struct NftRepository<'r>(&'r DatabaseConnection);

impl<'r> NftRepository<'r> {
    pub fn new(conn: &'r DatabaseConnection) -> Self {
        Self(conn)
    }

    pub async fn find_active_nfts_with_position_range(
        &self,
        start: u32,
        end: u32,
    ) -> Result<Vec<nft::Model>, DbErr> {
        Nft::find()
            .filter(nft::Column::IsActive.eq(true))
            .filter(nft::Column::Position.between(start, end))
            .all(self.0)
            .await
    }
}
