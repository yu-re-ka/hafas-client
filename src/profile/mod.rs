#[cfg(feature = "db-profile")]
pub mod db;
#[cfg(feature = "sncf-profile")]
pub mod sncf;

use std::collections::HashMap;
use ijson::IValue;
use chrono::FixedOffset;
use chrono::NaiveDate;
use chrono::DateTime;
use geojson::Feature;

use crate::ParseResult;
use crate::TariffClass;
use crate::Leg;
use crate::Line;
use crate::Operator;
use crate::Place;
use crate::Remark;
use crate::Stopover;
use crate::Products;
use crate::Product;
use crate::Journey;

use crate::api::journeys::JourneysResponse;
use crate::api::locations::LocationsResponse;

use crate::parse::location::*;
use crate::parse::products::*;
use crate::parse::date::*;
use crate::parse::line::*;
use crate::parse::operator::*;
use crate::parse::leg::*;
use crate::parse::stopover::*;
use crate::parse::arrival_or_departure::*;
use crate::parse::load_factor::*;
use crate::parse::remark::*;
use crate::parse::polyline::*;
use crate::parse::journey::*;
use crate::parse::journeys_response::*;
use crate::parse::locations_response::*;
use crate::parse::common::*;

pub trait Profile: Sized {
    fn url(&self) -> &'static str;
    fn salt(&self) -> &'static str;
    fn prepare_body(&self, req_json: &mut IValue);
    fn prepare_headers(&self, headers: &mut HashMap<&str, &str>);
    fn price_currency(&self) -> &'static str;

    fn parse_common(&self, data: HafasCommon, tariff_class: TariffClass) -> ParseResult<CommonData> { default_parse_common(self, data, tariff_class) }
    fn parse_arrival_or_departure(&self, data: HafasArrivalOrDeparture, date: &NaiveDate) -> ParseResult<ArrivalOrDeparture> { default_parse_arrival_or_departure(self, data, date) }
    fn parse_stopover(&self, data: HafasStopover, common: &CommonData, date: &NaiveDate) -> ParseResult<Stopover> { default_parse_stopover(self, data, common, date) }
    fn parse_remark(&self, data: HafasRemark) -> ParseResult<Remark> { default_parse_remark(data) }
    fn parse_products(&self, p_cls: u16) -> Products { default_parse_products(p_cls) }
    fn parse_product(&self, p_cls: u16) -> ParseResult<Product> { default_parse_product(p_cls) }
    fn parse_polyline(&self, data: HafasPolyline) -> ParseResult<Vec<Feature>> { default_parse_polyline(data) }
    fn parse_operator(&self, data: HafasOperator) -> ParseResult<Operator> { default_parse_operator(data) }
    fn parse_locations_response(&self, data: HafasLocationsResponse) -> ParseResult<LocationsResponse> { default_parse_locations_response(self, data) }
    fn parse_coords(&self, data: HafasCoords) -> (f32, f32) { default_parse_coords(data) }
    fn parse_place(&self, data: HafasPlace) -> ParseResult<Place> { default_parse_place(self, data) }
    fn parse_line(&self, data: HafasLine, operators: &Vec<Operator>) -> ParseResult<Line> { default_parse_line(self, data, operators) }
    fn parse_leg(&self, data: HafasLeg, common: &CommonData, date: &NaiveDate) -> ParseResult<Leg> { default_parse_leg(self, data, common, date) }
    fn parse_journeys_response(&self, data: HafasJourneysResponse, tariff_class: TariffClass) -> ParseResult<JourneysResponse> { default_parse_journeys_response(self, data, tariff_class) }
    fn parse_date(&self, time: Option<String>, tz_offset: Option<i32>, date: &NaiveDate) -> ParseResult<Option<DateTime<FixedOffset>>> { default_parse_date(time, tz_offset, date) }
    fn parse_load_factor_entry(&self, h: HafasLoadFactorEntry) -> ParseResult<LoadFactorEntry> { default_parse_load_factor_entry(h) }
    fn parse_journey(&self, data: HafasJourney, common: &CommonData) -> ParseResult<Journey> { default_parse_journey(self, data, common) }
}
