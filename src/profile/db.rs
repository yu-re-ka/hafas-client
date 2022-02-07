use ijson::{ijson, IValue};
use std::collections::HashMap;
use crate::Profile;

#[derive(Debug)]
pub struct DbProfile;

impl Profile for DbProfile {
    fn url(&self) -> &'static str { "https://reiseauskunft.bahn.de/bin/mgate.exe" }
    fn salt(&self) -> &'static str { "bdI8UVj40K5fvxwf" }

    fn prepare_body(&self, req_json: &mut IValue) {
        req_json["svcReqL"][0]["cfg"]["rtMode"] = ijson!("HYBRID");
        req_json["client"] = ijson!({
            "id": "DB",
            "v": "16040000",
            "type": "IPH",
            "name": "DB Navigator"
        });
        req_json["ext"] = ijson!("DB.R19.04.a");
        req_json["ver"] = ijson!("1.16");
        req_json["auth"] = ijson!({
            "type": "AID",
            "aid": "n91dB8Z77MLdoR0K"
        });
    }

    fn prepare_headers(&self, headers: &mut HashMap<&str, &str>) {
        headers.insert("User-Agent", "my-awesome-e5f276d8fe6cprogram");
    }

    fn price_currency(&self) -> &'static str { "EUR" }
}
