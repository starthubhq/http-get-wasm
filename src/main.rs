use std::io::{self, Read};
use std::time::Duration;

use serde_json::{json, Value};
use waki::Client;

fn main() {
    // ---- read stdin ----
    let mut buf = String::new();
    let _ = io::stdin().read_to_string(&mut buf);
    let input: Value = serde_json::from_str(&buf)
        .unwrap_or_else(|_| json!({}));

    // ---- required url (input should be an object with 'url' field) ----
    let Some(input_obj) = input.as_object() else {
        eprintln!(r#"{{"error": "input must be a JSON object"}}"#);
        return;
    };
    
    let Some(url) = input_obj.get("url").and_then(|v| v.as_str()) else {
        eprintln!(r#"{{"error": "missing required 'url' field"}}"#);
        return;
    };

    // ---- optional headers (headers field can be a string or object) ----
    let mut headers_static: Vec<(&'static str, &'static str)> = Vec::new();
    if let Some(headers_value) = input_obj.get("headers") {
        match headers_value {
            Value::String(headers_str) => {
                // Parse JSON string if it's a JSON string
                if let Ok(headers_obj) = serde_json::from_str::<Value>(headers_str) {
                    if let Some(headers_map) = headers_obj.as_object() {
                        for (k, v) in headers_map {
                            if let Some(val) = v.as_str() {
                                let k_static: &'static str = Box::leak(k.clone().into_boxed_str());
                                let v_static: &'static str = Box::leak(val.to_string().into_boxed_str());
                                headers_static.push((k_static, v_static));
                            }
                        }
                    }
                }
            },
            Value::Object(headers_obj) => {
                // Direct object format
                for (k, v) in headers_obj {
                    if let Some(val) = v.as_str() {
                        let k_static: &'static str = Box::leak(k.clone().into_boxed_str());
                        let v_static: &'static str = Box::leak(val.to_string().into_boxed_str());
                        headers_static.push((k_static, v_static));
                    }
                }
            },
            _ => {}
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

            // Success: output to stdout as key-value object
            let output = json!({
                "status": status,
                "body": body_str
            });
            
            println!("{}", output.to_string());
        }
        Err(e) => {
            // Error: output to stderr as key-value object
            let error_output = json!({
                "error": e.to_string()
            });
            eprintln!("{}", error_output.to_string());
        }
    }
}