use crate::ParseResult;
use crate::TariffClass;
use crate::parse::journey::HafasJourney;
use crate::parse::journey::parse_journey;
use crate::parse::common::parse_common;
use crate::parse::common::HafasCommon;
use serde::Deserialize;
use crate::api::journeys::JourneysResponse;

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct HafasJourneysResponse {
    out_ctx_scr_b: Option<String>,
    out_ctx_scr_f: Option<String>,
    out_con_l: Vec<HafasJourney>,
    common: HafasCommon,
}

pub fn parse_journeys_response(data: HafasJourneysResponse, tariff_class: TariffClass) -> ParseResult<JourneysResponse> {
    let HafasJourneysResponse { out_ctx_scr_b, out_ctx_scr_f, out_con_l, common } = data;
    let common_data = parse_common(common, tariff_class)?;

    Ok(JourneysResponse {
        earlier_ref: out_ctx_scr_b,
        later_ref: out_ctx_scr_f,
        journeys: out_con_l.into_iter().map(|x| parse_journey(x, &common_data)).collect::<ParseResult<_>>()?,
    })
}
