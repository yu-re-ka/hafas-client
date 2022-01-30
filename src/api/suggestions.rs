use ijson::ijson;
use crate::{Result, Client, Profile, Requester, Place};
use crate::client::HafasClient;
use crate::parse::suggestions_response::parse_suggestions_response;
use crate::parse::suggestions_response::HafasSuggestionsResponse;

pub type SuggestionsResponse = Vec<Place>;

impl<P: Profile + Sync + Send, R: Requester + Sync + Send> HafasClient<P, R> {
    pub async fn suggestions(&self, query: &str, results: Option<u64>) -> Result<SuggestionsResponse> {
        let data: HafasSuggestionsResponse = self.request(ijson!({
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

        Ok(parse_suggestions_response(data)?)
    }
}
