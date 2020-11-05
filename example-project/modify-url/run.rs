use crate::net::fetch;
use crate::fmt::print;
use serde_json::{Value, Map};

struct Request {
    url: String,
    body: String,
    state: Map<String, Value>,
}

#[no_mangle]
pub fn run(input: Vec<u8>) -> Option<Vec<u8>> {
    let req = match request(input) {
        Some(r) => r,
        None => return Some(String::from("failed").as_bytes().to_vec())
    };

	let modified = format!("{}/suborbital", req.body.as_str());
    Some(modified.as_bytes().to_vec())
}

fn request(input: Vec<u8>) -> Option<Request> {
    let v: Value = match serde_json::from_slice(&input) {
        Ok(val) => val,
        Err(e) => {
            print(format!("failed to unmarshal request: {}", e).as_str());
            return None
        },
    };

    let url = String::from(v["url"].as_str().unwrap());
    let body = String::from(v["body"].as_str().unwrap());
    let state_map = v["state"].as_object().unwrap();

    let req = Request{ url: url, body: body, state: state_map.to_owned() };
    Some(req)
}