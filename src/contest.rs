fn fetch_contest_info() -> serde_json::Value {
    let url = "https://abc-latest.deno.dev/";
    let res = ureq::get(url).set("Accept", "application/json").call();
    let res = res.unwrap();
    let body = res.into_string().unwrap();
    let body: serde_json::Value = serde_json::from_str(&body).unwrap();
    body
}

pub fn get_contest_info() -> String {
    let body = fetch_contest_info();
    let contest_time = body["start"].as_str().unwrap();
    let contest_title = body["title"].as_str().unwrap();
    let contest_url = body["url"].as_str().unwrap();
    let contest_info = format!("**AtCoder Beginner Contest開催情報**\n**コンテスト名** : {contest_title:}\n**開始日時** : {contest_time:}\n{contest_url:}"
    );
    contest_info
}
