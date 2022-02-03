use crate::ParseResult;
use crate::Place;
use crate::Line;
use crate::TariffClass;
use crate::Remark;
use crate::parse::location::HafasPlace;
use crate::parse::location::parse_place;
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

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub(crate) struct HafasCommon {
    loc_l: Vec<HafasPlace>,
    prod_l: Vec<HafasLine>,
    op_l: Vec<HafasOperator>,
    tcoc_l: Option<Vec<HafasLoadFactorEntry>>,
    rem_l: Vec<HafasRemark>,
    poly_l: Vec<HafasPolyline>,
}

#[derive(Debug)]
pub(crate) struct CommonData {
    pub tariff_class: TariffClass,
    pub places: Vec<Option<Place>>,
    pub lines: Vec<Option<Line>>,
    pub load_factors: Vec<LoadFactorEntry>,
    pub remarks: Vec<Remark>,
    pub polylines: Vec<Vec<geojson::Feature>>,
}

pub(crate) fn parse_common(data: HafasCommon, tariff_class: TariffClass) -> ParseResult<CommonData> {
    let HafasCommon { loc_l, prod_l, op_l, tcoc_l, rem_l, poly_l } = data;
    let operators = op_l.into_iter().map(|x| parse_operator(x)).collect::<ParseResult<_>>()?;
    Ok(CommonData {
        tariff_class,
        places: loc_l.into_iter().map(|x| parse_place(x).ok()).collect(),
        lines: prod_l.into_iter().map(|x| parse_line(x, &operators).ok()).collect(),
        load_factors: tcoc_l.unwrap_or(vec![]).into_iter().map(|x| parse_load_factor_entry(x)).collect::<ParseResult<_>>()?,
        remarks: rem_l.into_iter().map(|x| parse_remark(x)).collect::<ParseResult<_>>()?,
        polylines: poly_l.into_iter().map(|x| parse_polyline(x)).collect::<ParseResult<_>>()?,
    })
}
