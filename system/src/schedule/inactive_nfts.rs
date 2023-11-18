use chrono::{Local, NaiveDateTime};
use sea_orm::{sea_query::Expr, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};
use server::shared::database::entities::{nft, prelude::Nft};

pub async fn inactive_nfts(db: &DatabaseConnection) -> Result<(), DbErr> {
    let now = NaiveDateTime::from_timestamp_millis(Local::now().timestamp_millis());

    Nft::update_many()
        .col_expr(nft::Column::IsActive, Expr::value(false))
        .filter(nft::Column::EndDate.lt(now))
        .exec(db)
        .await?;

    Ok(())
}
