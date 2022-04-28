

pub fn api_get_request(url: &str) -> String {

    let body = reqwest::blocking::get(url).expect("unable to call api")
    .text().expect("unable to get text");

    return body
}
