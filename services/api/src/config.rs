use base64::{Engine as _, engine::general_purpose::STANDARD};
use std::{collections::HashMap, net::SocketAddr, time::Duration};
use url::Url;
#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub bind: SocketAddr,
    pub s3_endpoint: String,
    pub s3_key: String,
    pub s3_secret: String,
}

#[derive(Clone)]
pub struct HostedConfig {
    pub database_url: String,
    pub bind: SocketAddr,
    pub issuer: String,
    pub audience: String,
    pub google_client_id: String,
    pub jwt_key: Vec<u8>,
    pub refresh_key: Vec<u8>,
    pub access_ttl: Duration,
    pub refresh_ttl: Duration,
}

impl HostedConfig {
    pub fn from_env() -> Result<Self, &'static str> {
        Self::from_values(&std::env::vars().collect())
    }

    pub fn from_values(values: &HashMap<String, String>) -> Result<Self, &'static str> {
        let get = |key: &str| {
            values
                .get(key)
                .cloned()
                .ok_or("Missing required hosted authentication configuration.")
        };
        let database_url = get("DATABASE_URL")?;
        let database = Url::parse(&database_url).map_err(|_| "Invalid database URL.")?;
        if !matches!(database.scheme(), "postgres" | "postgresql")
            || database.host_str().is_none()
            || database.path().trim_matches('/').is_empty()
        {
            return Err("Hosted authentication requires PostgreSQL.");
        }
        let bind: SocketAddr = get("TRO_API_BIND")?
            .parse()
            .map_err(|_| "Invalid API bind address.")?;
        if !bind.ip().is_loopback() {
            return Err("Bind the hosted API behind a trusted HTTPS reverse proxy on loopback.");
        }
        let issuer_url =
            Url::parse(&get("TRO_AUTH_ISSUER")?).map_err(|_| "Invalid authentication issuer.")?;
        if issuer_url.scheme() != "https"
            || issuer_url.host_str().is_none()
            || !issuer_url.username().is_empty()
            || issuer_url.password().is_some()
            || issuer_url.query().is_some()
            || issuer_url.fragment().is_some()
        {
            return Err("Authentication issuer must be a fixed HTTPS origin.");
        }
        let issuer = issuer_url.as_str().trim_end_matches('/').to_owned();
        let audience = get("TRO_AUTH_AUDIENCE")?;
        if audience.is_empty() || audience.len() > 128 {
            return Err("Invalid authentication audience.");
        }
        let google_client_id = get("TRO_GOOGLE_CLIENT_ID")?;
        if google_client_id.len() > 255
            || !google_client_id.ends_with(".apps.googleusercontent.com")
        {
            return Err("Invalid Google desktop client ID.");
        }
        let jwt_key = decode_key(&get("TRO_JWT_KEY_B64")?)?;
        let refresh_key = decode_key(&get("TRO_REFRESH_KEY_B64")?)?;
        let access_seconds = parse_seconds(values, "TRO_ACCESS_TTL_SECONDS", 900)?;
        let refresh_seconds = parse_seconds(values, "TRO_REFRESH_TTL_SECONDS", 30 * 24 * 60 * 60)?;
        if !(5 * 60..=60 * 60).contains(&access_seconds) {
            return Err("Access-token lifetime must be between 5 and 60 minutes.");
        }
        if !(24 * 60 * 60..=30 * 24 * 60 * 60).contains(&refresh_seconds) {
            return Err("Refresh lifetime must be between 1 and 30 days.");
        }
        Ok(Self {
            database_url,
            bind,
            issuer,
            audience,
            google_client_id,
            jwt_key,
            refresh_key,
            access_ttl: Duration::from_secs(access_seconds),
            refresh_ttl: Duration::from_secs(refresh_seconds),
        })
    }
}

fn decode_key(value: &str) -> Result<Vec<u8>, &'static str> {
    let decoded = STANDARD
        .decode(value)
        .map_err(|_| "Authentication keys must be base64 encoded.")?;
    if decoded.len() < 32 {
        return Err("Authentication keys must contain at least 32 bytes.");
    }
    Ok(decoded)
}

fn parse_seconds(
    values: &HashMap<String, String>,
    key: &str,
    default: u64,
) -> Result<u64, &'static str> {
    values
        .get(key)
        .map(|value| {
            value
                .parse()
                .map_err(|_| "Invalid authentication lifetime.")
        })
        .unwrap_or(Ok(default))
}
impl Config {
    pub fn from_env() -> Result<Self, &'static str> {
        Self::from_values(&std::env::vars().collect(), cfg!(debug_assertions))
    }
    pub fn from_values(
        values: &HashMap<String, String>,
        debug: bool,
    ) -> Result<Self, &'static str> {
        if !debug || values.get("TRO_FIXTURE_MODE").map(String::as_str) != Some("1") {
            return Err("P0 API supports explicit development fixtures only.");
        }
        let get = |key: &str| {
            values
                .get(key)
                .cloned()
                .ok_or("Missing required fixture configuration.")
        };
        let database_url = get("DATABASE_URL")?;
        let database = Url::parse(&database_url).map_err(|_| "Invalid database URL.")?;
        if !matches!(database.scheme(), "postgres" | "postgresql")
            || database.host_str() != Some("127.0.0.1")
            || database.path() != "/tro_rebuild_test"
        {
            return Err("Refusing a non-isolated fixture database.");
        }
        let bind: SocketAddr = get("TRO_API_BIND")?
            .parse()
            .map_err(|_| "Invalid API bind address.")?;
        if !bind.ip().is_loopback() {
            return Err("Fixture API must bind to loopback.");
        }
        let s3_endpoint = get("TRO_S3_ENDPOINT")?;
        let s3 = Url::parse(&s3_endpoint).map_err(|_| "Invalid S3 endpoint.")?;
        if s3.scheme() != "http"
            || s3.host_str() != Some("127.0.0.1")
            || s3.path() != "/"
            || !s3.username().is_empty()
        {
            return Err("Fixture storage must use loopback.");
        }
        Ok(Self {
            database_url,
            bind,
            s3_endpoint,
            s3_key: get("TRO_S3_KEY")?,
            s3_secret: get("TRO_S3_SECRET")?,
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn values() -> HashMap<String, String> {
        [
            ("TRO_FIXTURE_MODE", "1"),
            (
                "DATABASE_URL",
                "postgres://fixture:unused@127.0.0.1/tro_rebuild_test",
            ),
            ("TRO_API_BIND", "127.0.0.1:4318"),
            ("TRO_S3_ENDPOINT", "http://127.0.0.1:19000"),
            ("TRO_S3_KEY", "fixture"),
            ("TRO_S3_SECRET", "unused"),
        ]
        .into_iter()
        .map(|(k, v)| (k.into(), v.into()))
        .collect()
    }
    #[test]
    fn rejects_unsafe_environments() {
        let mut env = values();
        assert!(Config::from_values(&env, true).is_ok());
        assert!(Config::from_values(&env, false).is_err());
        env.insert("DATABASE_URL".into(), "postgres://host/production".into());
        assert!(Config::from_values(&env, true).is_err());
        let mut env = values();
        env.insert("TRO_API_BIND".into(), "0.0.0.0:4318".into());
        assert!(Config::from_values(&env, true).is_err());
        env.remove("TRO_FIXTURE_MODE");
        assert!(Config::from_values(&env, true).is_err());
    }

    fn hosted_values() -> HashMap<String, String> {
        [
            ("DATABASE_URL", "postgres://tro:secret@db.example.com/tro"),
            ("TRO_API_BIND", "127.0.0.1:4318"),
            ("TRO_AUTH_ISSUER", "https://api.tro.example"),
            ("TRO_AUTH_AUDIENCE", "tro-desktop-api"),
            (
                "TRO_GOOGLE_CLIENT_ID",
                "desktop-client.apps.googleusercontent.com",
            ),
            (
                "TRO_JWT_KEY_B64",
                "MDEyMzQ1Njc4OWFiY2RlZjAxMjM0NTY3ODlhYmNkZWY=",
            ),
            (
                "TRO_REFRESH_KEY_B64",
                "ZmVkY2JhOTg3NjU0MzIxMGZlZGNiYTk4NzY1NDMyMTA=",
            ),
        ]
        .into_iter()
        .map(|(key, value)| (key.to_owned(), value.to_owned()))
        .collect()
    }

    #[test]
    fn hosted_config_is_fail_closed_and_bounded() {
        let values = hosted_values();
        let config = HostedConfig::from_values(&values).unwrap();
        assert_eq!(config.access_ttl, Duration::from_secs(900));
        assert_eq!(config.refresh_ttl, Duration::from_secs(30 * 24 * 60 * 60));

        let mut unsafe_values = values.clone();
        unsafe_values.insert("TRO_AUTH_ISSUER".into(), "http://api.tro.example".into());
        assert!(HostedConfig::from_values(&unsafe_values).is_err());

        let mut short_key = values.clone();
        short_key.insert("TRO_JWT_KEY_B64".into(), "dG9vLXNob3J0".into());
        assert!(HostedConfig::from_values(&short_key).is_err());

        let mut long_refresh = values;
        long_refresh.insert(
            "TRO_REFRESH_TTL_SECONDS".into(),
            (31 * 24 * 60 * 60).to_string(),
        );
        assert!(HostedConfig::from_values(&long_refresh).is_err());
    }
}
