use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Account::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Account::Id).uuid().not_null().primary_key())
                    .col(
                        ColumnDef::new(Account::DisplayName)
                            .string_len(120)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Account::VerifiedEmail)
                            .string_len(254)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Account::EmailNormalized)
                            .string_len(254)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Account::Status)
                            .string_len(16)
                            .not_null()
                            .default("active"),
                    )
                    .col(
                        ColumnDef::new(Account::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Account::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("uq_accounts_email_normalized")
                    .table(Account::Table)
                    .col(Account::EmailNormalized)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(AuthIdentity::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AuthIdentity::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(AuthIdentity::AccountId).uuid().not_null())
                    .col(
                        ColumnDef::new(AuthIdentity::Provider)
                            .string_len(32)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(AuthIdentity::ProviderSubject)
                            .string_len(255)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(AuthIdentity::Issuer)
                            .string_len(255)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(AuthIdentity::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(AuthIdentity::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_auth_identities_account")
                            .from(AuthIdentity::Table, AuthIdentity::AccountId)
                            .to(Account::Table, Account::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("uq_auth_identities_provider_subject")
                    .table(AuthIdentity::Table)
                    .col(AuthIdentity::Provider)
                    .col(AuthIdentity::ProviderSubject)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(AuthSession::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AuthSession::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(AuthSession::AccountId).uuid().not_null())
                    .col(ColumnDef::new(AuthSession::FamilyId).uuid().not_null())
                    .col(ColumnDef::new(AuthSession::ParentId).uuid().null())
                    .col(ColumnDef::new(AuthSession::ReplacedById).uuid().null())
                    .col(ColumnDef::new(AuthSession::TokenDigest).binary().not_null())
                    .col(
                        ColumnDef::new(AuthSession::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(AuthSession::LastUsedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(AuthSession::ExpiresAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(AuthSession::RevokedAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(AuthSession::ReuseDetectedAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_auth_sessions_account")
                            .from(AuthSession::Table, AuthSession::AccountId)
                            .to(Account::Table, Account::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_auth_sessions_parent")
                            .from(AuthSession::Table, AuthSession::ParentId)
                            .to(AuthSession::Table, AuthSession::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_auth_sessions_replacement")
                            .from(AuthSession::Table, AuthSession::ReplacedById)
                            .to(AuthSession::Table, AuthSession::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;
        for (name, column, unique) in [
            (
                "uq_auth_sessions_token_digest",
                AuthSession::TokenDigest,
                true,
            ),
            ("ix_auth_sessions_family", AuthSession::FamilyId, false),
            ("ix_auth_sessions_account", AuthSession::AccountId, false),
        ] {
            let mut index = Index::create();
            index.name(name).table(AuthSession::Table).col(column);
            if unique {
                index.unique();
            }
            manager.create_index(index.to_owned()).await?;
        }
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AuthSession::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(AuthIdentity::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Account::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Account {
    Table,
    Id,
    DisplayName,
    VerifiedEmail,
    EmailNormalized,
    Status,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum AuthIdentity {
    Table,
    Id,
    AccountId,
    Provider,
    ProviderSubject,
    Issuer,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum AuthSession {
    Table,
    Id,
    AccountId,
    FamilyId,
    ParentId,
    ReplacedById,
    TokenDigest,
    CreatedAt,
    LastUsedAt,
    ExpiresAt,
    RevokedAt,
    ReuseDetectedAt,
}
