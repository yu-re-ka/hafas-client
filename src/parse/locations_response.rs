use crate::ParseResult;
use crate::parse::location::{HafasPlace, parse_place};
use serde::Deserialize;
use crate::api::locations::LocationsResponse;

#[derive(Debug, Deserialize)]
pub struct HafasLocationsResponse {
    r#match: HafasLocationsResponseMatch,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HafasLocationsResponseMatch {
    loc_l: Vec<HafasPlace>,
}

pub fn parse_locations_response(data: HafasLocationsResponse) -> ParseResult<LocationsResponse> {
    Ok(data.r#match.loc_l
        .into_iter()
        .filter_map(|p| parse_place(p).ok())
        .collect::<Vec<_>>())
}
