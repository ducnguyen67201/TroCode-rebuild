use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "fixture_accounts")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub profile: String,
    pub role: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::fixture_session::Entity")]
    FixtureSession,
}

impl Related<super::fixture_session::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FixtureSession.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
