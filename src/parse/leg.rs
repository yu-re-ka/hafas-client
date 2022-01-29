use crate::Leg;
use crate::Result;
use crate::Error;
use serde::Deserialize;
use chrono::NaiveDate;
use crate::parse::journeys_response::CommonData;
use crate::parse::stopover::{HafasStopover, parse_stopover};
use crate::parse::arrival_or_departure::{HafasArrivalOrDeparture, parse_arrival_or_departure};
use crate::parse::polyline::parse_polyline;
use geojson::FeatureCollection;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HafasLegJnyPolyG {
    poly_x_l: Vec<usize>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HafasLegJny {
    jid: String,
    is_rchbl: Option<bool>,
    dir_txt: Option<String>,
    prod_x: Option<usize>,
    stop_l: Option<Vec<HafasStopover>>,
    msg_l: Vec<HafasLegJnyMsg>,
    poly_g: Option<HafasLegJnyPolyG>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HafasLegJnyMsg {
    rem_x: usize,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HafasLegArr {
    a_t_z_offset: Option<i32>,
    a_time_s: Option<String>,
    a_time_r: Option<String>,
    a_platf_s: Option<String>,
    a_platf_r: Option<String>,
    a_cncl: Option<bool>,
    loc_x: usize,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HafasLegDep {
    d_t_z_offset: Option<i32>,
    d_time_s: Option<String>,
    d_time_r: Option<String>,
    d_platf_s: Option<String>,
    d_platf_r: Option<String>,
    d_cncl: Option<bool>,
    loc_x: usize,
}

#[derive(Debug, Deserialize)]
pub struct HafasLeg {
    dep: HafasLegDep,
    arr: HafasLegArr,
    jny: Option<HafasLegJny>,
}

pub fn parse_leg(data: HafasLeg, common: &CommonData, date: &NaiveDate) -> Result<Leg> {
    let HafasLeg { dep, arr, jny } = data;
    let origin = common.places.get(dep.loc_x).ok_or(Error::InvalidData)?.clone();
    let destination = common.places.get(arr.loc_x).ok_or(Error::InvalidData)?.clone();
    let dep = parse_arrival_or_departure(HafasArrivalOrDeparture {
        t_z_offset: dep.d_t_z_offset,
        time_s: dep.d_time_s,
        time_r: dep.d_time_r,
        platf_s: dep.d_platf_s,
        platf_r: dep.d_platf_r,
        cncl: dep.d_cncl,
    }, date)?;
    let arr = parse_arrival_or_departure(HafasArrivalOrDeparture {
        t_z_offset: arr.a_t_z_offset,
        time_s: arr.a_time_s,
        time_r: arr.a_time_r,
        platf_s: arr.a_platf_s,
        platf_r: arr.a_platf_r,
        cncl: arr.a_cncl,
    }, date)?;

    let mut cancelled = None;
    if let Some(true) = dep.cancelled { cancelled = Some(true); }
    if let Some(true) = arr.cancelled { cancelled = Some(true); }

    let mut line = None;
    let mut reachable = None;
    let mut trip_id = None;
    let mut direction = None;
    let mut stopovers = None;
    let mut remarks = None;
    let mut polyline = None;
    if let Some(jny) = jny {
        let HafasLegJny { prod_x, is_rchbl, jid, dir_txt, stop_l, msg_l, poly_g } = jny;
        line = prod_x.and_then(|x| common.lines.get(x)).cloned();
        reachable = is_rchbl;
        trip_id = Some(jid);
        direction = dir_txt;
        stopovers = stop_l.map(|x| x.into_iter().map(|x| parse_stopover(x, common, date)).collect::<Result<_>>()).transpose()?;
        remarks = Some(msg_l.into_iter().filter_map(|x| common.remarks.get(x.rem_x).cloned()).collect());
        polyline = poly_g.map(|x| {
            let features = x.poly_x_l.into_iter().filter_map(|x| common.polylines.get(x).cloned()).flatten().collect();
            FeatureCollection {
                features,
                bbox: None,
                foreign_members: None,
            }
        })
    }

    Ok(Leg {
        origin,
        destination,
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
        cancelled,
        line,
        reachable,
        trip_id,
        direction,
        stopovers,
        load_factor: common.load_factors.iter().find(|x| x.class == common.tariff_class).map(|x| x.load),
        remarks,
        polyline,
    })
}
