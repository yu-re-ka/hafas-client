use crate::ParseResult;
use crate::Error;
use crate::Stopover;
use chrono::NaiveDate;
use serde::Deserialize;
use crate::parse::journeys_response::CommonData;
use crate::parse::arrival_or_departure::{HafasArrivalOrDeparture, parse_arrival_or_departure};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HafasStopover {
    loc_x: usize,
    a_t_z_offset: Option<i32>,
    a_time_s: Option<String>,
    a_time_r: Option<String>,
    a_platf_s: Option<String>,
    a_platf_r: Option<String>,
    a_cncl: Option<bool>,
    d_t_z_offset: Option<i32>,
    d_time_s: Option<String>,
    d_time_r: Option<String>,
    d_platf_s: Option<String>,
    d_platf_r: Option<String>,
    d_cncl: Option<bool>,
}

pub fn parse_stopover(data: HafasStopover, common: &CommonData, date: &NaiveDate) -> ParseResult<Stopover> {
    let HafasStopover { loc_x, a_t_z_offset, a_time_s, a_time_r, a_platf_s, a_platf_r, a_cncl, d_t_z_offset, d_time_s, d_time_r, d_platf_s, d_platf_r, d_cncl } = data;
    let stop = common.places.get(loc_x).ok_or_else(|| format!("Invalid place index {}", loc_x))?.clone();
    let dep = parse_arrival_or_departure(HafasArrivalOrDeparture {
        t_z_offset: d_t_z_offset,
        time_s: d_time_s,
        time_r: d_time_r,
        platf_s: d_platf_s,
        platf_r: d_platf_r,
        cncl: d_cncl,
    }, date)?;
    let arr = parse_arrival_or_departure(HafasArrivalOrDeparture {
        t_z_offset: a_t_z_offset,
        time_s: a_time_s,
        time_r: a_time_r,
        platf_s: a_platf_s,
        platf_r: a_platf_r,
        cncl: a_cncl,
    }, date)?;

    Ok(Stopover {
        stop,
        departure: dep.time,
        planned_departure: dep.planned_time,
        departure_delay: dep.delay,
        arrival: arr.time,
        planned_arrival: arr.planned_time,
        arrival_delay: arr.delay,
        arrival_platform: arr.platform,
        planned_arrival_platform: arr.planned_platform,
        departure_platform: dep.platform,
        planned_departure_platform: dep.planned_platform,
    })
}
