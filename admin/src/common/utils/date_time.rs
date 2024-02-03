use chrono::{Duration, NaiveDateTime, DateTime, Utc, Local};

pub fn convert_timestamp_to_local_datetime(duration_secs: i64) -> DateTime<Local> {
    let duration = Duration::seconds(duration_secs).num_microseconds().unwrap();
    let naive_date = NaiveDateTime::from_timestamp_micros(duration).unwrap();
    let date: DateTime<Utc> = DateTime::from_naive_utc_and_offset(naive_date, Utc);
    date.with_timezone(&Local)
}

pub fn convert_datetime_short_readable(datetime: DateTime<Utc>) -> String {
    datetime.format("%Y-%m-%d %l:%M %P").to_string()
}

pub fn convert_datetime_long_readable(datetime: DateTime<Utc>) -> String {
    datetime.format("%b %d %Y %l:%M %P").to_string()
}