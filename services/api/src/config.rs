use std::{collections::HashMap, net::SocketAddr};
use url::Url;
#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub bind: SocketAddr,
    pub s3_endpoint: String,
    pub s3_key: String,
    pub s3_secret: String,
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
}
