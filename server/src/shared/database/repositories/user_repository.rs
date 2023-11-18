use crate::shared::database::entities::{prelude::User, user};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, DbErr, EntityTrait,
    QueryFilter,
};
pub struct UserRepository<'r>(&'r DatabaseConnection);

impl<'r> UserRepository<'r> {
    pub fn new(conn: &'r DatabaseConnection) -> Self {
        Self(conn)
    }

    pub async fn get_user_by_address(&self, address: String) -> Result<Option<user::Model>, DbErr> {
        User::find()
            .filter(user::Column::Address.eq(address))
            .one(self.0)
            .await
    }

    pub async fn upsert_user(&self, address: String) -> Result<user::Model, DbErr> {
        let user = self.get_user_by_address(address.to_owned()).await?;

        match user {
            Some(user) => Ok(user),
            None => {
                user::ActiveModel {
                    address: Set(address),
                    ..Default::default()
                }
                .insert(self.0)
                .await
            }
        }
    }
}
