use md5::{Md5, Digest};
use ijson::{ijson, IValue};
use super::{Profile, Requester, Result, Error};
use std::collections::HashMap;
use async_trait::async_trait;
use serde::Deserialize;
use serde::de::DeserializeOwned;

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
    async fn request<T: DeserializeOwned>(&self, req_json: IValue) -> Result<T>;
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
struct HafasResponse<T> {
    svc_res_l: Vec<HafasResponse2<T>>,
}
#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
struct HafasResponse2<T> {
    res: T,
    err: String,
}

#[async_trait]
impl<P: Profile + Sync + Send, R: Requester + Sync + Send> Client for HafasClient<P, R> {
    async fn request<T: DeserializeOwned>(&self, req_json: IValue) -> Result<T> {
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
        //eprintln!("{:#?}", serde_json::from_slice::<serde_json::Value>(&bytes));
        let mut data: HafasResponse<T> =  serde_json::from_slice(&bytes)?;
        let HafasResponse2 { res, err } = data.svc_res_l.remove(0);
        if err != "OK" { return Err(Error::Hafas(err.clone())) }

        Ok(res)
    }
}
