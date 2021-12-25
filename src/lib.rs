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
pub use error::{Error, Result};

use serde::Serialize;


/* Types */

#[derive(Debug, Clone, Serialize)]
pub struct Coordinates (f64, f64);

#[derive(Debug, Clone, Serialize)]
pub enum Location {
    Stop {
        id: String,
        name: String,
        coordinates: Coordinates,
        products: Products,
        //station: Option<Station>,
    },
    Address {
        address: String,
        coordinates: Coordinates,
    },
    Point {
        id: String,
        name: String,
        coordinates: Coordinates,
    },
}

#[derive(Debug, Clone, Serialize)]
pub struct Station {
    pub id: u64,
    pub name: String,
    pub coordinates: Coordinates,
    pub products: Products,
}

#[derive(Debug, Clone, Serialize)]
pub struct Products {
    pub national_exp: bool,
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

#[derive(Debug, Clone, Serialize)]
pub enum Product {
    NationalExp,
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

#[derive(Debug, Clone, Serialize)]
pub enum Accessibility {
    r#None,
    Partial,
    Complete,
}

impl Products {
    pub fn all() -> Products {
        Products {
            national_exp: true,
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
