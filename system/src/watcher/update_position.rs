use sea_orm::{ConnectionTrait, DatabaseConnection, DbErr, Statement};

pub async fn update_nfts_position(db: &DatabaseConnection) -> Result<(), DbErr> {
  db.execute(Statement::from_sql_and_values(
    sea_orm::DatabaseBackend::Postgres,
    r#"
      WITH nft_ext AS (
        SELECT
        "nft"."id",
        row_number() OVER (
          ORDER BY "nft"."square_price" DESC NULLS LAST
        ) AS "index"
        FROM "nft"
        WHERE "nft"."is_active" = true
      )
      UPDATE nft
      SET
        "last_position" = "position",
        "position" = (
          SELECT "nft_ext"."index"
          FROM "nft_ext"
          WHERE "nft"."id" = "nft_ext"."id"
          LIMIT $1
        )
      WHERE is_active = true
    "#,
    [1.into()],
  ))
  .await
  .map(|_| ())
}
