use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "workspace")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
    pub status: String,
    pub created_by_account_id: Uuid,
    pub created_at: TimeDateTimeWithTimeZone,
    pub updated_at: TimeDateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::workspace_membership::Entity")]
    Membership,
    #[sea_orm(has_many = "super::workspace_audit_event::Entity")]
    AuditEvent,
}

impl Related<super::workspace_membership::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Membership.def()
    }
}

impl Related<super::workspace_audit_event::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AuditEvent.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
