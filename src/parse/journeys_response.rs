use crate::ParseResult;
use crate::Place;
use crate::Line;
use crate::Operator;
use crate::TariffClass;
use crate::Remark;
use crate::parse::location::HafasPlace;
use crate::parse::location::parse_place;
use crate::parse::journey::HafasJourney;
use crate::parse::journey::parse_journey;
use crate::parse::line::HafasLine;
use crate::parse::line::parse_line;
use crate::parse::operator::HafasOperator;
use crate::parse::operator::parse_operator;
use crate::parse::load_factor::HafasLoadFactorEntry;
use crate::parse::load_factor::LoadFactorEntry;
use crate::parse::remark::parse_remark;
use crate::parse::remark::HafasRemark;
use crate::parse::polyline::HafasPolyline;
use crate::parse::polyline::parse_polyline;
use crate::parse::load_factor::parse_load_factor_entry;
use serde::Deserialize;
use crate::api::journeys::JourneysResponse;

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct HafasJourneysResponse {
    out_ctx_scr_b: Option<String>,
    out_ctx_scr_f: Option<String>,
    out_con_l: Vec<HafasJourney>,
    common: HafasJourneysResponseCommon,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
struct HafasJourneysResponseCommon {
    loc_l: Vec<HafasPlace>,
    prod_l: Vec<HafasLine>,
    op_l: Vec<HafasOperator>,
    tcoc_l: Option<Vec<HafasLoadFactorEntry>>,
    rem_l: Vec<HafasRemark>,
    poly_l: Vec<HafasPolyline>,
}

#[derive(Debug)]
pub struct CommonData {
    pub tariff_class: TariffClass,
    pub places: Vec<Place>,
    pub lines: Vec<Line>,
    pub load_factors: Vec<LoadFactorEntry>,
    pub operators: Vec<Operator>,
    pub remarks: Vec<Remark>,
    pub polylines: Vec<Vec<geojson::Feature>>,
}

pub fn parse_journeys_response(data: HafasJourneysResponse, tariff_class: TariffClass) -> ParseResult<JourneysResponse> {
    let HafasJourneysResponse { out_ctx_scr_b, out_ctx_scr_f, out_con_l, common } = data;
    let common_data = {
        let HafasJourneysResponseCommon { loc_l, prod_l, op_l, tcoc_l, rem_l, poly_l } = common;
        let operators = op_l.into_iter().map(|x| parse_operator(x)).collect::<ParseResult<_>>()?;
        CommonData {
            tariff_class,
            places: loc_l.into_iter().filter_map(|x| parse_place(x).transpose()).collect::<ParseResult<_>>()?,
            lines: prod_l.into_iter().map(|x| parse_line(x, &operators)).collect::<ParseResult<_>>()?,
            load_factors: tcoc_l.unwrap_or(vec![]).into_iter().map(|x| parse_load_factor_entry(x)).collect::<ParseResult<_>>()?,
            remarks: rem_l.into_iter().map(|x| parse_remark(x)).collect::<ParseResult<_>>()?,
            polylines: poly_l.into_iter().map(|x| parse_polyline(x)).collect::<ParseResult<_>>()?,
            operators,
        }
    };

    Ok(JourneysResponse {
        earlier_ref: out_ctx_scr_b,
        later_ref: out_ctx_scr_f,
        journeys: out_con_l.into_iter().map(|x| parse_journey(x, &common_data)).collect::<ParseResult<_>>()?,
    })
}
