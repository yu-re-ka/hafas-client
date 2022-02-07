use ijson::ijson;
use crate::{Result, Client, Profile, Requester, Place};
use crate::client::HafasClient;
use crate::parse::locations_response::HafasLocationsResponse;
use serde::Serialize;
use serde::Deserialize;

pub type LocationsResponse = Vec<Place>;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct LocationsOptions {
    pub query: String,
    pub results: Option<u64>,
    pub language: Option<String>,
}

impl<P: Profile + Sync + Send, R: Requester + Sync + Send> HafasClient<P, R> {
    pub async fn locations(&self, opts: LocationsOptions) -> Result<LocationsResponse> {
        let data: HafasLocationsResponse = self.request(ijson!({
            "svcReqL": [
                {
                    "cfg": {
                        "polyEnc": "GPA"
                    },
                    "meth": "LocMatch",
                    "req": {
                        "input": {
                            "loc": {
                                "type": "ALL",
                                "name": format!("{}?", opts.query),
                            },
                            "maxLoc": opts.results.unwrap_or(10),
                            "field": "S"
                        }
                    }
                }
            ],
            "lang": opts.language.as_deref().unwrap_or("en"),
        })).await?;

        Ok(self.profile.parse_locations_response(data)?)
    }
}
