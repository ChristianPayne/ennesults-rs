use chrono::{
    DateTime, Duration, FixedOffset, Local, NaiveDateTime, ParseResult, TimeDelta, TimeZone, Utc,
};
// use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

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

// RFC2822
// Mon, 14 Oct 2024 23:36:13 -0700

// rfc3339
// 2024-10-14T23:36:43.116263-07:00

// let raw_date_time = "Mon, 14 Oct 2024 23:36:13 -0700";
// match parse_date_time(raw_date_time) {
//     Ok(date_time) => {
//         let reference_date_time = get_date_time_minutes_ago(5);
//         dbg!(&reference_date_time, &date_time);
//         let res = date_time_is_greater_than_reference(reference_date_time, date_time.into());
//         println!("is in range: {}", res);
//     }
//     Err(e) => {
//         println!("Error: {}", e);
//     }
// }
