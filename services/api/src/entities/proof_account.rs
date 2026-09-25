use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "proof_accounts")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub role: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::proof_session::Entity")]
    ProofSession,
    #[sea_orm(has_many = "super::runtime_grant::Entity")]
    RuntimeGrant,
}

impl Related<super::proof_session::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ProofSession.def()
    }
}

impl Related<super::runtime_grant::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RuntimeGrant.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
