#![feature(backtrace)]

pub mod error;
pub mod profile;
pub mod requester;
pub mod client;
pub mod api;
pub mod parse;
pub mod format;

pub use client::Client;
pub use profile::Profile;
pub use requester::Requester;
pub use error::{Error, Result, ParseError, ParseResult};
use chrono::FixedOffset;
use chrono::DateTime;
use geojson::FeatureCollection;

use serde::{Serialize, Deserialize};

/* Types */

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Location {
    Address {
        address: String,
        latitude: f32,
        longitude: f32,
    },
    Point {
        #[serde(skip_serializing_if = "Option::is_none")]
        id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        poi: Option<bool>,
        latitude: f32,
        longitude: f32,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum Place {
    Stop(Stop),
    Location(Location),
}

/*#[derive(Debug, Clone, Serialize)]
pub struct Station {
    pub id: u64,
    pub name: String,
    pub coordinates: Coordinates,
    pub products: Products,
}*/

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stop {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub location: Option<Location>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub products: Option<Products>,
    //station: Option<Station>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Products {
    pub national_express: bool,
    pub national: bool,
    pub regional_exp: bool,
    pub regional: bool,
    pub suburban: bool,
    pub bus: bool,
    pub ferry: bool,
    pub subway: bool,
    pub tram: bool,
    pub taxi: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Product {
    NationalExpress,
    National,
    RegionalExp,
    Regional,
    Suburban,
    Bus,
    Ferry,
    Subway,
    Tram,
    Taxi,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    Train,
    Bus,
    Watercraft,
    Taxi,
    Walking
}

impl Product {
    fn mode(&self) -> Mode {
        match *self {
            Product::NationalExpress => Mode::Train,
            Product::National => Mode::Train,
            Product::RegionalExp => Mode::Train,
            Product::Regional => Mode::Train,
            Product::Suburban => Mode::Train,
            Product::Bus => Mode::Bus,
            Product::Ferry => Mode::Watercraft,
            Product::Subway => Mode::Train,
            Product::Tram => Mode::Train,
            Product::Taxi => Mode::Taxi,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum TariffClass {
    First,
    Second,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Accessibility {
    r#None,
    Partial,
    Complete,
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum LoadFactor {
    LowToMedium,
    High,
    VeryHigh,
    ExceptionallyHigh,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Line {
    name: Option<String>,
    fahrt_nr: Option<String>,
    mode: Mode,
    product: Product,
    operator: Option<Operator>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Leg {
    origin: Place,
    destination: Place,
    departure: Option<DateTime<FixedOffset>>,
    planned_departure: Option<DateTime<FixedOffset>>,
    departure_delay: Option<u64>,
    arrival: Option<DateTime<FixedOffset>>,
    planned_arrival: Option<DateTime<FixedOffset>>,
    arrival_delay: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reachable: Option<bool>,
    trip_id: Option<String>,
    line: Option<Line>,
    direction: Option<String>,
    //current_location,
    arrival_platform: Option<String>,
    planned_arrival_platform: Option<String>,
    departure_platform: Option<String>,
    planned_departure_platform: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cancelled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stopovers: Option<Vec<Stopover>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    load_factor: Option<LoadFactor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    remarks: Option<Vec<Remark>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    polyline: Option<FeatureCollection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    walking: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transfer: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    distance: Option<u64>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Stopover {
    stop: Place,
    departure: Option<DateTime<FixedOffset>>,
    planned_departure: Option<DateTime<FixedOffset>>,
    departure_delay: Option<u64>,
    arrival: Option<DateTime<FixedOffset>>,
    planned_arrival: Option<DateTime<FixedOffset>>,
    arrival_delay: Option<u64>,
    arrival_platform: Option<String>,
    planned_arrival_platform: Option<String>,
    departure_platform: Option<String>,
    planned_departure_platform: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Journey {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    pub legs: Vec<Leg>,
    //last_updated
}

#[derive(Debug, Clone, Serialize)]
pub struct Operator {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum RemarkType {
    Hint,
    Status,
}

#[derive(Debug, Clone, Serialize)]
pub struct Remark {
    pub code: String,
    pub text: String,
    pub r#type: RemarkType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trip_id: Option<String>,
}

impl Products {
    pub fn all() -> Products {
        Products {
            national_express: true,
            national: true,
            regional_exp: true,
            regional: true,
            suburban: true,
            bus: true,
            ferry: true,
            subway: true,
            tram: true,
            taxi: true,
        }
    }
}
