//! Explicit operator-only provisioning. No public endpoint and no plaintext token logs.
use crate::{
    auth::digest,
    entities::{proof_account, proof_session},
};
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set, TransactionTrait,
    entity::prelude::ChronoDateTimeUtc, sea_query::Expr,
};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};
use uuid::Uuid;

fn utc_now() -> ChronoDateTimeUtc {
    SystemTime::now().into()
}

pub enum Command {
    Issue { origin: String, output: PathBuf },
    Revoke { account: Uuid },
}
pub fn parse(args: &[String]) -> Result<Command, &'static str> {
    match args {
        [command, flag, origin, output_flag, output]
            if command == "proof-issue" && flag == "--origin" && output_flag == "--output" =>
        {
            let url = url::Url::parse(origin).map_err(|_| "Invalid HTTPS proof origin.")?;
            if url.scheme() != "https"
                || url.host_str().is_none()
                || !url.username().is_empty()
                || url.password().is_some()
                || url.path() != "/"
                || url.query().is_some()
                || url.fragment().is_some()
            {
                return Err("A root HTTPS proof origin is required.");
            }
            Ok(Command::Issue {
                origin: origin.trim_end_matches('/').into(),
                output: output.into(),
            })
        }
        [command, flag, account] if command == "proof-revoke" && flag == "--account" => {
            Ok(Command::Revoke {
                account: Uuid::parse_str(account).map_err(|_| "Invalid account UUID.")?,
            })
        }
        _ => Err(
            "Use proof-issue --origin HTTPS_ORIGIN --output NEW_FILE or proof-revoke --account UUID.",
        ),
    }
}
fn private_file(path: &Path) -> Result<std::fs::File, &'static str> {
    let mut options = std::fs::OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let file = options
        .open(path)
        .map_err(|_| "Cannot create a new private output file.")?;
    #[cfg(windows)]
    {
        // Lock down the empty file before placing any credential in it.
        let restricted = (|| {
            let system = PathBuf::from(std::env::var_os("SYSTEMROOT").ok_or(())?).join("System32");
            let identity = std::process::Command::new(system.join("whoami.exe"))
                .args(["/user", "/fo", "csv", "/nh"])
                .output()
                .map_err(|_| ())?;
            if !identity.status.success() {
                return Err(());
            }
            let identity = String::from_utf8(identity.stdout).map_err(|_| ())?;
            let sid = identity
                .split('"')
                .find(|v| {
                    v.starts_with("S-1-")
                        && v.chars()
                            .all(|c| c.is_ascii_digit() || c == '-' || c == 'S')
                })
                .ok_or(())?;
            let result = std::process::Command::new(system.join("icacls.exe"))
                .arg(path)
                .args(["/inheritance:r", "/grant:r", &format!("*{sid}:F")])
                .output()
                .map_err(|_| ())?;
            if !result.status.success() {
                return Err(());
            }
            Ok(())
        })();
        if restricted.is_err() {
            drop(file);
            let _ = std::fs::remove_file(path);
            return Err("Cannot restrict the private output file ACL.");
        }
    }
    Ok(file)
}
pub async fn execute(pool: &DatabaseConnection, command: Command) -> Result<Uuid, &'static str> {
    match command {
        Command::Revoke { account } => {
            proof_session::Entity::update_many()
                .col_expr(proof_session::Column::Revoked, Expr::value(true))
                .filter(proof_session::Column::AccountId.eq(account))
                .exec(pool)
                .await
                .map_err(|_| "Proof revocation failed.")?;
            Ok(account)
        }
        Command::Issue { origin, output } => {
            let mut file = private_file(&output)?;
            let account = Uuid::new_v4();
            let token = format!("{}{}", Uuid::new_v4().simple(), Uuid::new_v4().simple());
            let result = async {
                let tx = pool
                    .begin()
                    .await
                    .map_err(|_| "Proof transaction unavailable.")?;
                proof_account::Entity::insert(proof_account::ActiveModel {
                    id: Set(account),
                    role: Set("student".to_owned()),
                })
                .exec(&tx)
                .await
                .map_err(|_| "Proof identity creation failed.")?;
                proof_session::Entity::insert(proof_session::ActiveModel {
                    token_digest: Set(digest(&token)),
                    account_id: Set(account),
                    expires_at: Set(utc_now() + Duration::from_secs(3600)),
                    revoked: Set(false),
                })
                .exec(&tx)
                .await
                .map_err(|_| "Proof session creation failed.")?;
                let body = serde_json::to_vec(&serde_json::json!({"origin":origin,"token":token}))
                    .map_err(|_| "Private configuration encoding failed.")?;
                file.write_all(&body)
                    .and_then(|_| file.sync_all())
                    .map_err(|_| "Private output write failed.")?;
                tx.commit()
                    .await
                    .map_err(|_| "Proof transaction commit failed.")?;
                Ok(account)
            }
            .await;
            if result.is_err() {
                // Commit errors can be ambiguous: revoke any possibly committed identity.
                let _ = proof_session::Entity::update_many()
                    .col_expr(proof_session::Column::Revoked, Expr::value(true))
                    .filter(proof_session::Column::AccountId.eq(account))
                    .exec(pool)
                    .await;
                drop(file);
                let _ = std::fs::remove_file(output);
            }
            result
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parses_only_explicit_private_operations() {
        let args = |v: &[&str]| v.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        assert!(
            parse(&args(&[
                "proof-issue",
                "--origin",
                "https://proof.example",
                "--output",
                "new.json"
            ]))
            .is_ok()
        );
        for origin in [
            "http://example.com",
            "https://u:p@example.com",
            "https://example.com/path",
        ] {
            assert!(
                parse(&args(&[
                    "proof-issue",
                    "--origin",
                    origin,
                    "--output",
                    "new.json"
                ]))
                .is_err()
            );
        }
        assert!(
            parse(&args(&[
                "proof-revoke",
                "--account",
                &Uuid::new_v4().to_string()
            ]))
            .is_ok()
        );
        assert!(parse(&[]).is_err());
    }
    #[test]
    fn private_output_refuses_collision() {
        let path = std::env::temp_dir().join(format!("tro-proof-{}", Uuid::new_v4()));
        let file = private_file(&path).unwrap();
        assert!(private_file(&path).is_err());
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            assert_eq!(file.metadata().unwrap().permissions().mode() & 0o777, 0o600);
        }
        drop(file);
        std::fs::remove_file(path).unwrap();
    }
    #[tokio::test]
    #[ignore = "isolated fixture database; run through npm run test:integration"]
    async fn issued_identity_is_digest_only_and_revocable() {
        let config = crate::config::Config::from_env().unwrap();
        let pool = crate::db::connect(&config).await.unwrap();
        crate::db::migrate(&pool).await.unwrap();
        let path = std::env::temp_dir().join(format!("tro-proof-{}.json", Uuid::new_v4()));
        let account = execute(
            &pool,
            Command::Issue {
                origin: "https://proof.example".into(),
                output: path.clone(),
            },
        )
        .await
        .unwrap();
        let config: serde_json::Value =
            serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
        let token = config["token"].as_str().unwrap();
        assert_eq!(token.len(), 64);
        let stored = proof_session::Entity::find()
            .filter(proof_session::Column::AccountId.eq(account))
            .one(&pool)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(stored.token_digest, digest(token));
        assert!(
            execute(
                &pool,
                Command::Issue {
                    origin: "https://proof.example".into(),
                    output: path.clone()
                }
            )
            .await
            .is_err()
        );
        execute(&pool, Command::Revoke { account }).await.unwrap();
        let revoked = proof_session::Entity::find()
            .filter(proof_session::Column::AccountId.eq(account))
            .one(&pool)
            .await
            .unwrap()
            .unwrap()
            .revoked;
        assert!(revoked);
        proof_session::Entity::delete_many()
            .filter(proof_session::Column::AccountId.eq(account))
            .exec(&pool)
            .await
            .unwrap();
        proof_account::Entity::delete_by_id(account)
            .exec(&pool)
            .await
            .unwrap();
        std::fs::remove_file(path).unwrap();
    }
}
