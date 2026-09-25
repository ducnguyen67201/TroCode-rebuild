use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "runtime_grants")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub token_digest: Vec<u8>,
    pub session_digest: Vec<u8>,
    pub account_id: Uuid,
    pub teaching_session_id: Uuid,
    pub expires_at: ChronoDateTimeUtc,
    pub remaining_calls: i32,
    pub revoked: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::proof_session::Entity",
        from = "Column::SessionDigest",
        to = "super::proof_session::Column::TokenDigest",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    ProofSession,
    #[sea_orm(
        belongs_to = "super::proof_account::Entity",
        from = "Column::AccountId",
        to = "super::proof_account::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    ProofAccount,
}

impl Related<super::proof_session::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ProofSession.def()
    }
}

impl Related<super::proof_account::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ProofAccount.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
