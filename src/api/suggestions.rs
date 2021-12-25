use ijson::ijson;
use crate::{Result, Error, Client, Profile, Requester, Location};
use crate::client::HafasClient;
use crate::parse::parse_location;

impl<P: Profile + Sync + Send, R: Requester + Sync + Send> HafasClient<P, R> {
    pub async fn suggestions(&self, query: &str, results: Option<u64>) -> Result<Vec<Location>> {
        let data = self.request(ijson!({
            "cfg": {
                "polyEnc": "GPA"
            },
            "meth": "LocMatch",
            "req": {
                "input": {
                    "loc": {
                        "type": "ALL",
                        "name": format!("{}?", query),
                    },
                    "maxLoc": results.unwrap_or(10),
                    "field": "S"
                }
            }
        })).await?;

        data["svcResL"][0]["res"]["match"]["locL"].as_array().ok_or_else(|| Error::InvalidData)?
            .iter()
            .map(|p| parse_location(p.clone()))
            .collect::<Result<Vec<_>>>()
    }
}
