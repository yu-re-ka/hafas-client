use crate::ParseResult;
use crate::Profile;
use chrono::DateTime;
use chrono::NaiveDate;
use chrono::FixedOffset;

pub struct HafasArrivalOrDeparture {
    pub t_z_offset: Option<i32>,
    pub time_s: Option<String>,
    pub time_r: Option<String>,
    pub platf_s: Option<String>,
    pub platf_r: Option<String>,
    pub cncl: Option<bool>,
}

pub struct ArrivalOrDeparture {
    pub platform: Option<String>,
    pub planned_platform: Option<String>,
    pub time: Option<DateTime<FixedOffset>>,
    pub planned_time: Option<DateTime<FixedOffset>>,
    pub delay: Option<u64>,
    pub cancelled: Option<bool>,
}

pub(crate) fn default_parse_arrival_or_departure<P: Profile>(profile: &P, data: HafasArrivalOrDeparture, date: &NaiveDate) -> ParseResult<ArrivalOrDeparture> {
    let HafasArrivalOrDeparture { t_z_offset, time_s, time_r, platf_s, platf_r, cncl } = data;
    let planned_time = profile.parse_date(time_s, t_z_offset, date)?;
    let rt_time = profile.parse_date(time_r, t_z_offset, date)?;
    Ok(ArrivalOrDeparture {
        platform: platf_r.or(platf_s.clone()),
        planned_platform: platf_s,
        time: rt_time.or(planned_time),
        planned_time: planned_time,
        delay: planned_time.zip(rt_time).map(|(planned_time, rt_time)| {
            let diff = rt_time - planned_time;
            diff.num_seconds() as u64
        }),
        cancelled: cncl,
    })
}
