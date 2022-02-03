use ijson::{ijson, IValue};
use std::collections::HashMap;
use crate::Profile;

#[derive(Debug)]
pub struct SncfProfile;

impl Profile for SncfProfile {
    fn url(&self) -> &'static str { "https://sncf-maps.hafas.de/bin/maps-ng/mgate.exe" }
    fn salt(&self) -> &'static str { "bdI8UVj40K5fvxwf" }

    fn prepare_body(&self, req_json: &mut IValue) {
        //req_json["svcReqL"][0]["cfg"]["rtMode"] = ijson!("HYBRID");
        req_json["client"] = ijson!({
            "id": "SNCF_LIVEMAP",
            "type": "WEB",
            "name": "webapp",
            "l": "vs_webapp"
        });
        req_json["id"] = ijson!("6tm47gqmkkk7hgcs");
        req_json["ver"] = ijson!("1.18");
        req_json["auth"] = ijson!({
            "type": "AID",
            "aid": "hf7mcf9bv3nv8g5f"
        });
    }

    fn prepare_headers(&self, headers: &mut HashMap<&str, &str>) {
        headers.insert("User-Agent", "my-awesome-e5f276d8fe6cprogram");
    }
}
