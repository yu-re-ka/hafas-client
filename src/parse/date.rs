use crate::Error;
use crate::Result;
use chrono::TimeZone;
use chrono::NaiveTime;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::NaiveDate;
use chrono::Duration;
use chrono::LocalResult;

pub fn parse_date(time: Option<String>, tz_offset: Option<i32>, date: &NaiveDate) -> Result<Option<DateTime<FixedOffset>>> {
    let time = match time {
        Some(time) => time,
        None => return Ok(None),
    };
    let tz_offset = tz_offset.unwrap_or(60);

    let (dayoffset, time) = match time.len() {
        8 => {
            let iter = time.chars();
            let dayoffset = iter.clone().take(2).collect::<String>()
                .parse().map_err(|_| Error::InvalidData)?;
            (dayoffset, iter.skip(2).collect::<String>())
        },
        6 => (0, time),
        _ => return Err(Error::InvalidData),
    };

    let time = NaiveTime::parse_from_str(&time, "%H%M%S").map_err(|_| Error::InvalidData)?;
    let naive_dt = date.and_time(time) + Duration::days(dayoffset);
    let timezone = FixedOffset::east(tz_offset * 60);
    let dt = match timezone.from_local_datetime(&naive_dt) {
        LocalResult::Single(t) => t,
        _ => return Err(Error::InvalidData),
    };
    Ok(Some(dt))
}
