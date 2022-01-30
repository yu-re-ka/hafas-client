use crate::{ParseResult, Place, Location, Error};
use super::products::parse_products;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct HafasLocation {
    x: u64,
    y: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HafasPlace {
    r#type: Option<String>,
    name: String,
    crd: HafasLocation,
    ext_id: Option<String>,
    p_cls: Option<u16>,
}

fn parse_location(location: HafasLocation) -> Location {
    Location {
        latitude: location.x as f32 / 1000000.0,
        longitude: location.y as f32 / 1000000.0
    }
}

pub fn parse_place(data: HafasPlace) -> ParseResult<Option<Place>> {
    let HafasPlace { r#type, name, crd, ext_id, p_cls } = data;
    Ok(match r#type.as_deref() {
        Some("S") => Some(Place::Stop {
            location: parse_location(crd),
            id: ext_id.ok_or_else(|| "Missing ext_id")?,
            name: name,
            products: parse_products(p_cls.ok_or_else(|| "Missing p_cls")?),
        }),
        Some("P") => Some(Place::Point {
            location: parse_location(crd),
            id: ext_id.ok_or_else(|| "Missing ext_id")?,
            name: name,
        }),
        Some("A") => Some(Place::Address {
            location: parse_location(crd),
            address: name,
        }),
        _ => None
    })
}
