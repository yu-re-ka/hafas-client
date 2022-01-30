use crate::ParseResult;
use crate::parse::location::{HafasPlace, parse_place};
use serde::Deserialize;
use crate::api::suggestions::SuggestionsResponse;

#[derive(Debug, Deserialize)]
pub struct HafasSuggestionsResponse {
    r#match: HafasSuggestionsResponseMatch,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HafasSuggestionsResponseMatch {
    loc_l: Vec<HafasPlace>,
}

pub fn parse_suggestions_response(data: HafasSuggestionsResponse) -> ParseResult<SuggestionsResponse> {
    data.r#match.loc_l
        .into_iter()
        .filter_map(|p| parse_place(p).transpose())
        .collect::<ParseResult<Vec<_>>>()
}
