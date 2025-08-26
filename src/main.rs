use std::io::{self, Read};
use std::time::Duration;

use serde::Deserialize;
use serde_json::{json, Value};
use waki::Client;

#[derive(Deserialize)]
struct Input {
    #[serde(default)] state: Value,
    #[serde(default)] params: Value,
}

fn main() {
    // ---- read stdin ----
    let mut buf = String::new();
    let _ = io::stdin().read_to_string(&mut buf);
    let input: Input = serde_json::from_str(&buf)
        .unwrap_or(Input { state: Value::Null, params: Value::Null });

    // ---- required url ----
    let Some(url) = input.params.get("url").and_then(|v| v.as_str()) else {
        eprintln!("Error: missing required param 'url'");
        return;
    };

    // ---- optional headers (make &'static strs) ----
    let mut headers_static: Vec<(&'static str, &'static str)> = Vec::new();
    if let Some(hmap) = input.params.get("headers").and_then(|v| v.as_object()) {
        for (k, v) in hmap {
            if let Some(val) = v.as_str() {
                let k_static: &'static str = Box::leak(k.clone().into_boxed_str());
                let v_static: &'static str = Box::leak(val.to_string().into_boxed_str());
                headers_static.push((k_static, v_static));
            }
        }
    }

    // ---- GET ----
    let resp = Client::new()
        .get(url)
        .headers(headers_static) // <-- pass Vec, not slice
        .connect_timeout(Duration::from_secs(15))
        .send();

    match resp {
        Ok(r) => {
            let status = r.status_code();
            let body = r.body().unwrap_or_default();
            let body_str = String::from_utf8_lossy(&body).to_string();

            // Emit manifest-style outputs
            println!("::starthub:state::{}", json!({
                "status": status,
                "body": body_str
            }).to_string());

            eprintln!("GET {} -> {}", url, status);
        }
        Err(e) => eprintln!("Request error: {}", e),
    }
}
