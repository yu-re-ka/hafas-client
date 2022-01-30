use crate::ParseResult;
use chrono::TimeZone;
use chrono::NaiveTime;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::NaiveDate;
use chrono::Duration;

pub fn parse_date(time: Option<String>, tz_offset: Option<i32>, date: &NaiveDate) -> ParseResult<Option<DateTime<FixedOffset>>> {
    let time = match time {
        Some(time) => time,
        None => return Ok(None),
    };
    let tz_offset = tz_offset.unwrap_or(60);

    let (dayoffset, time) = match time.len() {
        8 => {
            let iter = time.chars();
            let dayoffset = iter.clone().take(2).collect::<String>()
                .parse()?;
            (dayoffset, iter.skip(2).collect::<String>())
        },
        6 => (0, time),
        len => return Err(format!("invalid time length. expected 6 or 8, got {}", len).into()),
    };

    let time = NaiveTime::parse_from_str(&time, "%H%M%S")?;
    let naive_dt = date.and_time(time) + Duration::days(dayoffset);
    let timezone = FixedOffset::east(tz_offset * 60);
    let dt = timezone.from_local_datetime(&naive_dt).unwrap(); // This will never panic for FixedOffset timezone
    Ok(Some(dt))
}
