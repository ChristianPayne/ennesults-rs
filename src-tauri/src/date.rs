use chrono::{DateTime, Duration, FixedOffset, Local, ParseResult};

pub fn get_local_now() -> DateTime<Local> {
    Local::now()
}

pub fn get_local_now_formatted() -> String {
    Local::now().to_rfc2822()
}

pub fn parse_date_time(date_time: &str) -> ParseResult<DateTime<FixedOffset>> {
    DateTime::parse_from_rfc2822(date_time)
}

pub fn get_date_time_minutes_ago(minutes: u32) -> DateTime<Local> {
    Local::now() - Duration::minutes(minutes.into())
}

pub fn date_time_is_greater_than_reference(
    reference_date_time: DateTime<Local>,
    value_to_check: DateTime<Local>,
) -> bool {
    reference_date_time < value_to_check
}
