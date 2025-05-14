# Website Status Checker (Rust)

## Build Instructions

To build in release mode:

```bash
cargo build --release
```

The executable will be generated at:

```
target/release/website-status-checker-rust
```

---

##  Usage

To run the program using a list of websites from a file:

```bash
cargo run -- --file sites.txt --timeout 5 --retries 1
```

### CLI Options:

- `--file <filename>`: A text file containing URLs, one per line.
- `--timeout <seconds>`: Timeout per request in seconds (default: 5).
- `--retries <count>`: Number of retry attempts for failed URLs (default: 0).

### Example `sites.txt` file:

```
https://example.com
https://notareal.website.zzz
```

---

##  Output

### Terminal Output:

Live results are printed as each check completes:

```
WebsiteStatus { 
  url: "https://example.com", 
  action_status: Ok(200), 
  response_time: 100.070072ms, 
  timestamp: SystemTime { tv_sec: ..., tv_nsec: ... } 
}
```

### JSON Output from status.json:

After all checks complete, results are saved in a structured JSON format:

```json
[
  {
    "url": "https://example.com",
    "status": 200,
    "response_time_ms": 100,
    "timestamp": 1747258322
  },
  {
    "url": "https://notareal.website.zzz",
    "status": "error sending request for url (https://notareal.website.zzz/): ...",
    "response_time_ms": 20,
    "timestamp": 1747258322
  }
]
```

---

##  Features & Enhancements

 **Multithreading**:  
Each URL is checked in a separate thread for maximum concurrency and speed.

 **Retry Mechanism**:  
The user can specify how many times to retry failed requests using `--retries`.

 **Custom Timeouts**:  
Each request is limited to a timeout (in seconds) set by `--timeout`.

 **File Input Parsing**:  
URLs are loaded from a text file passed with `--file`.

 **Manual JSON Writing**:  
Instead of using `serde`, a custom function creates and formats the output JSON.

 **Live Terminal Output**:  
Each website's status is printed in real time for quick feedback.

 **Timestamps**:  
Each result is timestamped with the system time when the check completed.

- **Rust**
- `reqwest` (for blocking HTTP requests)
- `std::thread` (for concurrency)
- `std::sync::mpsc` (for inter-thread communication)

---

##  How It Works

1. **Command-line parsing**: `config.rs` extracts URLs from the provided file and reads timeout and retry settings.

2. **Spawning threads**: Each URL is processed in a separate thread via `std::thread::spawn`.

3. **Request + retry loop**: Each thread tries the URL up to N times (as specified by `--retries`), measuring response time and capturing errors.

4. **Channel communication**: Threads send their results to the main thread using `std::sync::mpsc::channel`.

5. **Collect + write**: Once all threads are done, results are serialized into `status.json` using a manually constructed JSON string.

---
