use chrono::{FixedOffset, TimeZone, NaiveDateTime, Duration};

fn get_now_time() -> String {
    let time_difference = 9;
    let hour = 3600;
    FixedOffset::east_opt(time_difference * hour)
        .unwrap()
        .from_utc_datetime(&chrono::Utc::now().naive_utc())
        .format("%Y/%m/%d %H:%M:%S")
        .to_string()
}

fn get_contest_time() -> String {
    let url = "https://abc-latest.deno.dev/";
    let res = ureq::get(url)
        .set("Accept", "application/json")
        .call();
    let res = res.unwrap();
    let body = res.into_string().unwrap();
    let body: serde_json::Value = serde_json::from_str(&body).unwrap();
    let contest_time = body["start"].as_str().unwrap();
    contest_time.to_string()
}

fn is_one_hour_before_the_contest() -> bool {
    let contest_time = get_contest_time();
    let now_time = get_now_time();
    let contest_time = NaiveDateTime::parse_from_str(&contest_time, "%Y/%m/%d %H:%M:%S").unwrap();
    let now_time = NaiveDateTime::parse_from_str(&now_time, "%Y/%m/%d %H:%M:%S").unwrap();
    // let now_time = NaiveDateTime::parse_from_str("2023/01/07 20:03:55", "%Y/%m/%d %H:%M:%S").unwrap();
    let duration = contest_time - now_time;
    duration < Duration::hours(1)
}

pub fn result () -> String {
    is_one_hour_before_the_contest().to_string()
}
