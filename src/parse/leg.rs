use crate::Leg;
use crate::ParseResult;
use serde::Deserialize;
use chrono::NaiveDate;
use crate::parse::common::CommonData;
use crate::parse::stopover::{HafasStopover, parse_stopover};
use crate::parse::arrival_or_departure::{HafasArrivalOrDeparture, parse_arrival_or_departure};
use geojson::FeatureCollection;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HafasLegJnyPolyG {
    poly_x_l: Vec<usize>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HafasLegJnyLoad {
    tcoc_x: Vec<usize>,
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
    d_trn_cmp_s_x: Option<HafasLegJnyLoad>,
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
pub enum HafasLegType {
    #[serde(rename = "JNY")]
    Journey,
    #[serde(rename = "WALK")]
    Walk,
    #[serde(rename = "TRSF")]
    Transfer,
    #[serde(rename = "DEVI")]
    Devi,
}

#[derive(Debug, Deserialize)]
pub struct HafasLegGis {
    dist: u64,
}

#[derive(Debug, Deserialize)]
pub struct HafasLeg {
    dep: HafasLegDep,
    arr: HafasLegArr,
    jny: Option<HafasLegJny>,
    gis: Option<HafasLegGis>,
    r#type: HafasLegType,
}

pub(crate) fn parse_leg(data: HafasLeg, common: &CommonData, date: &NaiveDate) -> ParseResult<Leg> {
    let HafasLeg { dep, arr, jny, gis, r#type } = data;
    let origin = common.places.get(dep.loc_x).cloned()
        .ok_or_else(|| format!("Invalid place index: {}", arr.loc_x))?
        .ok_or_else(|| format!("Parse error place index: {}", arr.loc_x))?;
    let destination = common.places.get(arr.loc_x).cloned()
        .ok_or_else(|| format!("Invalid place index: {}", arr.loc_x))?
        .ok_or_else(|| format!("Parse error place index: {}", arr.loc_x))?;
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
    let mut load_factor = None;
    let mut remarks = None;
    let mut polyline = None;
    let mut is_walking = None;
    let mut is_transfer = None;
    let mut distance = None;

    match r#type {
        HafasLegType::Journey => {
            let HafasLegJny { prod_x, is_rchbl, jid, dir_txt, stop_l, msg_l, poly_g, d_trn_cmp_s_x } = jny.ok_or_else(|| "Missing jny field")?;
            line = prod_x.map(|x| -> ParseResult<_> {
                Ok(common.lines.get(x).cloned()
                    .ok_or_else(|| format!("Invalid line index: {}", x))?
                    .ok_or_else(|| format!("Parse error line index: {}", x))?)
            }).transpose()?;
            reachable = is_rchbl;
            trip_id = Some(jid);
            direction = dir_txt;
            stopovers = stop_l.map(|x| x.into_iter().map(|x| parse_stopover(x, common, date)).collect::<ParseResult<_>>()).transpose()?;
            remarks = Some(msg_l.into_iter().map(|x| {
                common.remarks.get(x.rem_x).cloned()
                    .ok_or_else(|| format!("Invalid remark index: {}", x.rem_x).into())
            }).collect::<ParseResult<_>>()?);
            polyline = poly_g.map(|poly_g| -> ParseResult<_> {
                let mut features = vec![];
                for x in poly_g.poly_x_l {
                    let mut polyline = common.polylines.get(x).ok_or_else(|| format!("Invalid polyline index: {}", x))?.clone();
                    features.append(&mut polyline);
                }
                Ok(FeatureCollection { features, bbox: None, foreign_members: None })
            }).transpose()?;
            load_factor = d_trn_cmp_s_x.map(|x: HafasLegJnyLoad| -> ParseResult<_> {
                let mut entries = vec![];
                for i in x.tcoc_x {
                    entries.push(common.load_factors.get(i).ok_or_else(|| format!("Invalid load factor index: {}", i))?.clone());
                }
                Ok(entries.into_iter().find(|x| x.class == common.tariff_class).map(|x| x.load))
            }).transpose()?.and_then(|x| x);
        },
        HafasLegType::Walk => {
            is_walking = Some(true);
            distance = Some(gis.ok_or_else(|| "missing field gis")?.dist);
        },
        HafasLegType::Transfer | HafasLegType::Devi => {
            is_transfer = Some(true);
        },
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
        load_factor,
        remarks,
        polyline,
        is_walking,
        is_transfer,
        distance,
    })
}
