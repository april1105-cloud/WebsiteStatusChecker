use std::fs::File;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::result::WebsiteStatus;

pub fn write_status_json(results: &[WebsiteStatus]) -> std::io::Result<()> {
    let mut json = String::from("[\n");

    for (i, result) in results.iter().enumerate() {
        let timestamp = result.timestamp
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let entry = match &result.action_status {
            Ok(code) => format!(
                "  {{ \"url\": \"{}\", \"status\": {}, \"response_time_ms\": {}, \"timestamp\": {} }}",
                result.url,
                code,
                result.response_time.as_millis(),
                timestamp
            ),
            Err(err) => format!(
                "  {{ \"url\": \"{}\", \"status\": \"{}\", \"response_time_ms\": {}, \"timestamp\": {} }}",
                result.url,
                err,
                result.response_time.as_millis(),
                timestamp
            ),
        };

        json.push_str(&entry);
        if i != results.len() - 1 {
            json.push_str(",\n");
        } else {
            json.push('\n');
        }
    }

    json.push_str("]\n");

    let mut file = File::create("status.json")?;
    file.write_all(json.as_bytes())?;
    Ok(())
}
