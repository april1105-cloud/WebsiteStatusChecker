use std::time::{Duration, SystemTime};

#[derive(Debug)]
pub struct WebsiteStatus {
    pub url: String,
    pub action_status: Result<u16, String>,
    pub response_time: Duration,
    pub timestamp: SystemTime,
}

impl WebsiteStatus {
    pub fn to_json_object(&self) -> String {
        let status = match &self.action_status {
            Ok(code) => format!("{code}"),
            Err(e) => format!("\"{e}\""),
        };

        format!(
            "{{\"url\": \"{}\", \"status\": {}, \"response_time_ms\": {}, \"timestamp\": {:?}}}",
            self.url,
            status,
            self.response_time.as_millis(),
            self.timestamp.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()
        )
    }
}
