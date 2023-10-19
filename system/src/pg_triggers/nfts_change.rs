use sea_orm::{ConnectionTrait, DatabaseConnection, DbErr, ExecResult, Statement};

pub async fn create_nfts_change_event(db_con: &DatabaseConnection) -> Result<ExecResult, DbErr> {
  db_con
    .execute(Statement::from_sql_and_values(
      sea_orm::DatabaseBackend::Postgres,
      r#"
            CREATE OR REPLACE FUNCTION nfts_change_listener()
            RETURNS TRIGGER AS $$
            BEGIN
                IF TG_OP = 'INSERT' OR TG_OP = 'UPDATE' THEN
                    PERFORM pg_notify('nfts_change', row_to_json(NEW)::text);
                ELSE          
                    PERFORM pg_notify('nfts_change', row_to_json(OLD)::text);
                END IF;
                RETURN NEW;
            END;
            $$ LANGUAGE plpgsql;
        "#,
      [],
    ))
    .await?;

  db_con
    .execute(Statement::from_sql_and_values(
      sea_orm::DatabaseBackend::Postgres,
      r#"
            CREATE OR REPLACE TRIGGER nfts_change 
            AFTER 
                INSERT 
                OR DELETE 
                OR UPDATE OF square_price, is_active
            ON nft
            FOR EACH ROW 
            EXECUTE PROCEDURE nfts_change_listener();
        "#,
      [],
    ))
    .await
}
