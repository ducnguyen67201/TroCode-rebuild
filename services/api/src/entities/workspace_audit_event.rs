use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "workspace_audit_event")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub actor_account_id: Uuid,
    pub target_membership_id: Uuid,
    pub action: String,
    pub created_at: TimeDateTimeWithTimeZone,
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
        from = "Column::ActorAccountId",
        to = "super::account::Column::Id",
        on_update = "NoAction",
        on_delete = "Restrict"
    )]
    ActorAccount,
    #[sea_orm(
        belongs_to = "super::workspace_membership::Entity",
        from = "Column::TargetMembershipId",
        to = "super::workspace_membership::Column::Id",
        on_update = "NoAction",
        on_delete = "Restrict"
    )]
    TargetMembership,
}

impl Related<super::workspace::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Workspace.def()
    }
}

impl Related<super::workspace_membership::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TargetMembership.def()
    }
}

impl Related<super::account::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ActorAccount.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
