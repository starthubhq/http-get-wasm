use std::io::{self, Read};
use std::time::Duration;

use serde_json::{json, Value};
use waki::Client;

fn main() {
    // ---- read stdin ----
    let mut buf = String::new();
    let _ = io::stdin().read_to_string(&mut buf);
    let input: Vec<Value> = serde_json::from_str(&buf)
        .unwrap_or_else(|_| vec![]);

    // ---- required url (first element should be an object with 'url' field) ----
    let Some(url_obj) = input.get(0).and_then(|v| v.as_object()) else {
        eprintln!(r#"{{"error": "missing required first element with 'url' field"}}"#);
        return;
    };
    
    let Some(url) = url_obj.get("url").and_then(|v| v.as_str()) else {
        eprintln!(r#"{{"error": "missing required 'url' field in first element"}}"#);
        return;
    };

    // ---- optional headers (second element should be an object with headers) ----
    let mut headers_static: Vec<(&'static str, &'static str)> = Vec::new();
    if let Some(headers_obj) = input.get(1).and_then(|v| v.as_object()) {
        for (k, v) in headers_obj {
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

            // Success: output to stdout
            let output = json!([
                {"status": status},
                {"body": body_str}
            ]);
            
            println!("{}", output.to_string());
        }
        Err(e) => {
            // Error: output to stderr
            let error_output = json!([
                {"error": e.to_string()}
            ]);
            eprintln!("{}", error_output.to_string());
        }
    }
}