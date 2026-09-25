use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "workspace_membership")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub account_id: Option<Uuid>,
    pub email: String,
    pub email_normalized: String,
    pub role: String,
    pub added_by_account_id: Uuid,
    pub created_at: TimeDateTimeWithTimeZone,
    pub joined_at: Option<TimeDateTimeWithTimeZone>,
    pub removed_at: Option<TimeDateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::workspace::Entity",
        from = "Column::WorkspaceId",
        to = "super::workspace::Column::Id",
        on_update = "NoAction",
        on_delete = "Restrict"
    )]
    Workspace,
    #[sea_orm(
        belongs_to = "super::account::Entity",
        from = "Column::AccountId",
        to = "super::account::Column::Id",
        on_update = "NoAction",
        on_delete = "Restrict"
    )]
    Account,
    #[sea_orm(
        belongs_to = "super::account::Entity",
        from = "Column::AddedByAccountId",
        to = "super::account::Column::Id",
        on_update = "NoAction",
        on_delete = "Restrict"
    )]
    AddedByAccount,
    #[sea_orm(has_many = "super::workspace_audit_event::Entity")]
    AuditEvent,
}

impl Related<super::workspace::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Workspace.def()
    }
}

impl Related<super::account::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Account.def()
    }
}

impl Related<super::workspace_audit_event::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AuditEvent.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
