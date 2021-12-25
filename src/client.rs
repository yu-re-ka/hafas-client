use md5::{Md5, Digest};
use ijson::{ijson, IValue};
use super::{Profile, Requester, Result, Error};
use std::collections::HashMap;
use async_trait::async_trait;

#[derive(Debug)]
pub struct HafasClient<P: Profile + Sync + Send, R: Requester + Sync + Send> {
    profile: P,
    requester: R,
}

impl<P: Profile + Sync + Send, R: Requester + Sync + Send> HafasClient<P, R> {
    pub fn new(profile: P, requester: R) -> Self {
        HafasClient {
            profile,
            requester,
        }
    }
}

#[async_trait]
pub trait Client {
    async fn request(&self, req_json: IValue) -> Result<IValue>;
}

#[async_trait]
impl<P: Profile + Sync + Send, R: Requester + Sync + Send> Client for HafasClient<P, R> {
    async fn request(&self, req_json: IValue) -> Result<IValue> {
		let mut req_json = ijson!({
			"lang": "de",
            "svcReqL": vec![req_json],
        });
        self.profile.prepare_body(&mut req_json);
        let req_str = serde_json::to_string(&req_json)?;

        let mut hasher = Md5::new();
        hasher.update(&req_str);
        hasher.update(self.profile.salt());
        let checksum = hasher.finalize()
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<_>>()
            .join("");

        let url = format!("{}?checksum={}", self.profile.url(), checksum);

        let mut headers = HashMap::new();
        headers.insert("Content-Type", "application/json");
        headers.insert("Accept", "application/json");
        self.profile.prepare_headers(&mut headers);

        let bytes = self.requester.request(url, req_str, headers).await?;
        let data: IValue = serde_json::from_slice(&bytes)?;

        let status: &str = data["svcResL"][0]["err"].as_string().ok_or_else(|| Error::InvalidData)?;
        if status != "OK" { return Err(Error::Hafas(status.to_string())) }

        Ok(data)
    }
}
