use ijson::ijson;
use crate::{Result, Client, Profile, Requester, Place};
use crate::client::HafasClient;
use crate::parse::locations_response::parse_locations_response;
use crate::parse::locations_response::HafasLocationsResponse;

pub type LocationsResponse = Vec<Place>;

impl<P: Profile + Sync + Send, R: Requester + Sync + Send> HafasClient<P, R> {
    pub async fn locations(&self, query: &str, results: Option<u64>) -> Result<LocationsResponse> {
        let data: HafasLocationsResponse = self.request(ijson!({
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

        Ok(parse_locations_response(data)?)
    }
}
