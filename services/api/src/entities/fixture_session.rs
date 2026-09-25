use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "fixture_sessions")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub token_digest: Vec<u8>,
    pub account_id: Uuid,
    pub expires_at: ChronoDateTimeUtc,
    pub revoked: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::fixture_account::Entity",
        from = "Column::AccountId",
        to = "super::fixture_account::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    FixtureAccount,
}

impl Related<super::fixture_account::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FixtureAccount.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
