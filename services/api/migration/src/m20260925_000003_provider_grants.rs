use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ProviderGrant::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ProviderGrant::TokenDigest)
                            .binary_len(32)
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ProviderGrant::Kind)
                            .string_len(16)
                            .not_null(),
                    )
                    .col(ColumnDef::new(ProviderGrant::AccountId).uuid().not_null())
                    .col(
                        ColumnDef::new(ProviderGrant::AuthSessionId)
                            .uuid()
                            .not_null(),
                    )
                    .col(ColumnDef::new(ProviderGrant::SubjectId).uuid().not_null())
                    .col(
                        ColumnDef::new(ProviderGrant::ExpiresAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProviderGrant::RemainingRequests)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProviderGrant::RemainingAudioMs)
                            .big_integer()
                            .null(),
                    )
                    .col(ColumnDef::new(ProviderGrant::LastSequence).integer().null())
                    .col(
                        ColumnDef::new(ProviderGrant::Revoked)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(ProviderGrant::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_provider_grant_account")
                            .from(ProviderGrant::Table, ProviderGrant::AccountId)
                            .to(Account::Table, Account::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_provider_grant_auth_session")
                            .from(ProviderGrant::Table, ProviderGrant::AuthSessionId)
                            .to(AuthSession::Table, AuthSession::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .check((
                        "ck_provider_grant_kind",
                        Expr::col(ProviderGrant::Kind).is_in(["voice", "agent"]),
                    ))
                    .check((
                        "ck_provider_grant_requests",
                        Expr::col(ProviderGrant::RemainingRequests).gte(0),
                    ))
                    .check((
                        "ck_provider_grant_audio",
                        Expr::col(ProviderGrant::RemainingAudioMs)
                            .is_null()
                            .or(Expr::col(ProviderGrant::RemainingAudioMs).gte(0)),
                    ))
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("uq_provider_grant_subject")
                    .table(ProviderGrant::Table)
                    .col(ProviderGrant::Kind)
                    .col(ProviderGrant::AuthSessionId)
                    .col(ProviderGrant::SubjectId)
                    .unique()
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_provider_grant_expiry")
                    .table(ProviderGrant::Table)
                    .col(ProviderGrant::ExpiresAt)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ProviderGrant::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ProviderGrant {
    Table,
    TokenDigest,
    Kind,
    AccountId,
    AuthSessionId,
    SubjectId,
    ExpiresAt,
    RemainingRequests,
    RemainingAudioMs,
    LastSequence,
    Revoked,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Account {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum AuthSession {
    Table,
    Id,
}
