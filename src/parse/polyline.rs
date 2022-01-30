use crate::ParseResult;
use geojson::{Feature, Value, Geometry};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HafasPolylineLocRef {
    pp_idx: usize,
    loc_x: usize,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HafasPolyline {
    crd_enc_y_x: String,
    pp_loc_ref_l: Vec<HafasPolylineLocRef>,
}

pub fn parse_polyline(data: HafasPolyline) -> ParseResult<Vec<Feature>> {
    let HafasPolyline { crd_enc_y_x, pp_loc_ref_l } = data;
    let coords = polyline::decode_polyline(&crd_enc_y_x, 5)?;

    let features = coords.into_points().into_iter()
        .map(|point| {
            Feature::from(Geometry::new(Value::Point(vec![point.x(), point.y()])))
        })
        .collect();

    Ok(features)
}
