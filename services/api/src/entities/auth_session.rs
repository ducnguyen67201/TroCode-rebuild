use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "auth_session")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub account_id: Uuid,
    pub family_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub replaced_by_id: Option<Uuid>,
    pub token_digest: Vec<u8>,
    pub created_at: TimeDateTimeWithTimeZone,
    pub last_used_at: TimeDateTimeWithTimeZone,
    pub expires_at: TimeDateTimeWithTimeZone,
    pub revoked_at: Option<TimeDateTimeWithTimeZone>,
    pub reuse_detected_at: Option<TimeDateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::account::Entity",
        from = "Column::AccountId",
        to = "super::account::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Account,
}

impl Related<super::account::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Account.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
