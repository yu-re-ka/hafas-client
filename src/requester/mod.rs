#[cfg(feature = "hyper-requester")]
pub mod hyper;

use async_trait::async_trait;
use std::collections::HashMap;
use crate::Result;

#[async_trait]
pub trait Requester {
    async fn request(&self, url: String, body: String, headers: HashMap<&str, &str>) -> Result<Vec<u8>>;
}
