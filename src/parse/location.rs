use crate::{Result, Location, Coordinates};
use super::products::parse_products;
use serde::Deserialize;
use ijson::IValue;

#[derive(Debug, Deserialize)]
struct HafasCoordinates {
    x: u64,
    y: u64,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum HafasLocation {
    #[serde(rename(deserialize = "S"))]
    #[serde(rename_all = "camelCase")]
    Stop {
        ext_id: String,
        name: String,
        crd: HafasCoordinates,
        p_cls: u16,
    },
    #[serde(rename(deserialize = "P"))]
    #[serde(rename_all = "camelCase")]
    Point {
        ext_id: String,
        name: String,
        crd: HafasCoordinates,
    },
    #[serde(rename(deserialize = "A"))]
    #[serde(rename_all = "camelCase")]
    Address {
        name: String,
        crd: HafasCoordinates,
    },
}

fn parse_coordinates(coordinates: HafasCoordinates) -> Coordinates {
    Coordinates (coordinates.x as f64 / 1000000.0, coordinates.y as f64 / 1000000.0)
}

pub fn parse_location(val: IValue) -> Result<Location> {
    let hafas_point: HafasLocation = ijson::from_value(&val)?;

    Ok(match hafas_point {
        HafasLocation::Stop { ext_id, name, crd, p_cls } => Location::Stop {
            coordinates: parse_coordinates(crd),
            id: ext_id,
            name: name,
            products: parse_products(p_cls),
        },
        HafasLocation::Point { ext_id, name, crd } => Location::Point {
            coordinates: parse_coordinates(crd),
            id: ext_id,
            name: name,
        },
        HafasLocation::Address { name, crd } => Location::Address {
            coordinates: parse_coordinates(crd),
            address: name,
        },
    })
}
