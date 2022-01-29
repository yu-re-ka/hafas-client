use crate::{Result, Place, Location};
use super::products::parse_products;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct HafasLocation {
    x: u64,
    y: u64,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum HafasPlace {
    #[serde(rename(deserialize = "S"))]
    #[serde(rename_all = "camelCase")]
    Stop {
        ext_id: String,
        name: String,
        crd: HafasLocation,
        p_cls: u16,
    },
    #[serde(rename(deserialize = "P"))]
    #[serde(rename_all = "camelCase")]
    Point {
        ext_id: String,
        name: String,
        crd: HafasLocation,
    },
    #[serde(rename(deserialize = "A"))]
    #[serde(rename_all = "camelCase")]
    Address {
        name: String,
        crd: HafasLocation,
    },
}

fn parse_location(location: HafasLocation) -> Location {
    Location {
        latitude: location.x as f32 / 1000000.0,
        longitude: location.y as f32 / 1000000.0
    }
}

pub fn parse_place(hafas_point: HafasPlace) -> Result<Place> {
    Ok(match hafas_point {
        HafasPlace::Stop { ext_id, name, crd, p_cls } => Place::Stop {
            location: parse_location(crd),
            id: ext_id,
            name: name,
            products: parse_products(p_cls),
        },
        HafasPlace::Point { ext_id, name, crd } => Place::Point {
            location: parse_location(crd),
            id: ext_id,
            name: name,
        },
        HafasPlace::Address { name, crd } => Place::Address {
            location: parse_location(crd),
            address: name,
        },
    })
}
