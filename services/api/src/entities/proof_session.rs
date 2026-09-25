use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "proof_sessions")]
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
        belongs_to = "super::proof_account::Entity",
        from = "Column::AccountId",
        to = "super::proof_account::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    ProofAccount,
    #[sea_orm(has_many = "super::runtime_grant::Entity")]
    RuntimeGrant,
}

impl Related<super::proof_account::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ProofAccount.def()
    }
}

impl Related<super::runtime_grant::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RuntimeGrant.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
