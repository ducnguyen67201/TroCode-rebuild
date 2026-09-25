use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "account")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub display_name: String,
    pub verified_email: String,
    pub email_normalized: String,
    pub status: String,
    pub created_at: TimeDateTimeWithTimeZone,
    pub updated_at: TimeDateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::auth_identity::Entity")]
    AuthIdentity,
    #[sea_orm(has_many = "super::auth_session::Entity")]
    AuthSession,
}

impl Related<super::auth_identity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AuthIdentity.def()
    }
}

impl Related<super::auth_session::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AuthSession.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
