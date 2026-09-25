use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "provider_grant")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub token_digest: Vec<u8>,
    pub kind: String,
    pub account_id: Uuid,
    pub auth_session_id: Uuid,
    pub subject_id: Uuid,
    pub expires_at: TimeDateTimeWithTimeZone,
    pub remaining_requests: i32,
    pub remaining_audio_ms: Option<i64>,
    pub last_sequence: Option<i32>,
    pub revoked: bool,
    pub created_at: TimeDateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::account::Entity",
        from = "Column::AccountId",
        to = "super::account::Column::Id",
        on_delete = "Cascade"
    )]
    Account,
    #[sea_orm(
        belongs_to = "super::auth_session::Entity",
        from = "Column::AuthSessionId",
        to = "super::auth_session::Column::Id",
        on_delete = "Cascade"
    )]
    AuthSession,
}

impl Related<super::account::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Account.def()
    }
}

impl Related<super::auth_session::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AuthSession.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
