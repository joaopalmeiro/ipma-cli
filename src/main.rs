fn main() {
    let client = reqwest::blocking::Client::new();

    // More info:
    // - https://docs.rs/reqwest/0.11.3/reqwest/blocking/struct.RequestBuilder.html
    // - https://docs.rs/reqwest/0.11.3/reqwest/blocking/struct.Response.html
    // - https://docs.serde.rs/serde_json/value/enum.Value.html
    // - https://doc.rust-lang.org/std/fmt/
    let response = client
        .get(format!(
            "https://api.ipma.pt/open-data/forecast/meteorology/cities/daily/{id}.json",
            id = 1110600
        ))
        .send()
        .expect("a successful request")
        .json::<serde_json::Value>()
        .expect("expected the body to be json");
    dbg!(response);
}
