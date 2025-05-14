use std::env;
use std::fs;

pub struct Config {
    pub urls: Vec<String>,
    pub workers: usize,
    pub timeout_secs: u64,
    pub retries: u32,
    pub file: Option<String>,
}

impl Config {
    pub fn from_args() -> Result<Config, String> {
        let args: Vec<String> = env::args().collect();

        let mut file = None;
        let mut urls = vec![];
        let mut workers = num_cpus::get();
        let mut timeout_secs = 5;
        let mut retries = 0;

        let mut i = 1;
        while i < args.len() {
            match args[i].as_str() {
                "--file" => {
                    if i + 1 >= args.len() {
                        return Err("Expected filename after --file".to_string());
                    }
                    file = Some(args[i + 1].clone());
                    i += 2;
                }
                "--workers" => {
                    if i + 1 >= args.len() {
                        return Err("Expected number after --workers".to_string());
                    }
                    workers = args[i + 1].parse().map_err(|_| "Invalid number for --workers")?;
                    i += 2;
                }
                "--timeout" => {
                    if i + 1 >= args.len() {
                        return Err("Expected number after --timeout".to_string());
                    }
                    timeout_secs = args[i + 1].parse().map_err(|_| "Invalid number for --timeout")?;
                    i += 2;
                }
                "--retries" => {
                    if i + 1 >= args.len() {
                        return Err("Expected number after --retries".to_string());
                    }
                    retries = args[i + 1].parse().map_err(|_| "Invalid number for --retries")?;
                    i += 2;
                }
                other => {
                    urls.push(other.to_string());
                    i += 1;
                }
            }
        }

        // If a file was provided, read URLs from it
        if let Some(filename) = &file {
            let contents = fs::read_to_string(filename).map_err(|e| format!("Failed to read file: {}", e))?;
            for line in contents.lines() {
                let trimmed = line.trim();
                if !trimmed.is_empty() && !trimmed.starts_with('#') {
                    urls.push(trimmed.to_string());
                }
            }
        }

        if urls.is_empty() {
            return Err("No URLs provided via file or arguments.".to_string());
        }

        Ok(Config {
            urls,
            workers,
            timeout_secs,
            retries,
            file,
        })
    }
}
