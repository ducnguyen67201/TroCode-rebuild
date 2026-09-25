use crate::config::Config;
use object_store::{ObjectStore, ObjectStoreExt, aws::AmazonS3Builder, path::Path};
use std::{sync::Arc, time::Duration};
pub const BUCKET: &str = "tro-rebuild-fixtures";
pub const SAMPLE: &str = "materials/example.md";
pub fn connect(config: &Config) -> Result<Arc<dyn ObjectStore>, object_store::Error> {
    Ok(Arc::new(
        AmazonS3Builder::new()
            .with_bucket_name(BUCKET)
            .with_region("us-east-1")
            .with_endpoint(&config.s3_endpoint)
            .with_access_key_id(&config.s3_key)
            .with_secret_access_key(&config.s3_secret)
            .with_allow_http(true)
            .with_virtual_hosted_style_request(false)
            .build()?,
    ))
}
pub async fn ready(store: &dyn ObjectStore) -> bool {
    tokio::time::timeout(Duration::from_secs(2), store.head(&Path::from(SAMPLE)))
        .await
        .is_ok_and(|result| result.is_ok())
}
pub async fn seed(store: &dyn ObjectStore) -> Result<(), object_store::Error> {
    store
        .put(
            &Path::from(SAMPLE),
            include_bytes!("../../../tests/fixtures/materials/example.md")
                .as_slice()
                .into(),
        )
        .await?;
    Ok(())
}
