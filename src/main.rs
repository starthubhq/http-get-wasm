use std::io::{self, Read};
use std::time::Duration;

use serde_json::{json, Value};
use waki::Client;

fn main() {
    // ---- read stdin ----
    let mut buf = String::new();
    let _ = io::stdin().read_to_string(&mut buf);
    let input: Value = serde_json::from_str(&buf)
        .unwrap_or_else(|_| json!([]));

    // ---- parse new simplified array format inputs ----
    let Some(input_array) = input.as_array() else {
        eprintln!(r#"{{"error": "input must be a JSON array"}}"#);
        return;
    };
    
    // Extract url from position 0 (direct string)
    let Some(url_value) = input_array.get(0) else {
        eprintln!(r#"{{"error": "missing required URL at position 0"}}"#);
        return;
    };
    
    let Some(url) = url_value.as_str() else {
        eprintln!(r#"{{"error": "URL at position 0 must be a string"}}"#);
        return;
    };

    // ---- optional headers from position 1 (direct object) ----
    let mut headers_static: Vec<(&'static str, &'static str)> = Vec::new();
    if let Some(headers_input) = input_array.get(1) {
        if let Some(headers_map) = headers_input.as_object() {
            for (k, v) in headers_map {
                if let Some(val) = v.as_str() {
                    let k_static: &'static str = Box::leak(k.clone().into_boxed_str());
                    let v_static: &'static str = Box::leak(val.to_string().into_boxed_str());
                    headers_static.push((k_static, v_static));
                }
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

            // Success: output to stdout as simplified array format
            let output = json!([
                {
                    "status": status,
                    "body": body_str
                }
            ]);
            
            println!("{}", output.to_string());
        }
        Err(e) => {
            // Error: output to stderr as simplified array format
            let error_output = json!([
                {
                    "error": e.to_string()
                }
            ]);
            eprintln!("{}", error_output.to_string());
        }
    }
}