use crate::config::Config;
use crate::result::WebsiteStatus;
use crate::json_writer::write_status_json;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant, SystemTime};

use reqwest::blocking::Client;

pub fn run_checks(config: Config) -> Result<(), String> {
    let client = Client::builder()
        .timeout(Duration::from_secs(config.timeout_secs))
        .build()
        .map_err(|e| format!("Failed to build client: {}", e))?;

    let urls = config.urls;
    let (tx, rx) = mpsc::channel();
    let mut handles = vec![];

    println!("Starting checks...");

    for url in urls {
        let tx = tx.clone();
        let client = client.clone();
        let retries = config.retries;
        let url = url.clone();

        let handle = thread::spawn(move || {
            let mut last_err = None;
            let mut status = Err(String::new());
            let mut response_time = Duration::ZERO;

            for attempt in 0..=retries {
                let start = Instant::now();
                let result = client.get(&url).send();

                match result {
                    Ok(resp) => {
                        status = Ok(resp.status().as_u16());
                        response_time = start.elapsed();
                        break;
                    }
                    Err(e) => {
                        last_err = Some(e.to_string());
                        response_time = start.elapsed();
                        thread::sleep(Duration::from_millis(100));
                    }
                }
            }

            let final_status = status.or_else(|_| Err(last_err.unwrap_or("Unknown error".into())));
            let status = WebsiteStatus {
                url: url.clone(),
                action_status: final_status,
                response_time,
                timestamp: SystemTime::now(),
            };

            println!("{:?}", status); // live output
            tx.send(status).expect("Failed to send result");
        });

        handles.push(handle);
    }

    drop(tx); // close the sender

    let mut results = vec![];

    for received in rx {
        results.push(received);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Save to JSON
    write_status_json(&results).map_err(|e| format!("Failed to write JSON: {}", e))?;

    println!("All done. Results written to status.json.");
    Ok(())
}
