#[cfg(feature = "db-profile")]
pub mod db;

use std::collections::HashMap;
use ijson::IValue;

pub trait Profile {
    fn url(&self) -> &'static str;
    fn salt(&self) -> &'static str;
    fn prepare_body(&self, req_json: &mut IValue);
    fn prepare_headers(&self, headers: &mut HashMap<&str, &str>);
}
