use iron::prelude::*;
use iron::status;
use router::Router;
use std::io::prelude::*;
use std::fs::{File, create_dir_all};
use std::path::PathBuf;
use std::sync::RwLock;
use std::collections::HashMap;
use uuid::Uuid;
use time::{OffsetDateTime, Duration};
use base64::Engine;
use lazy_static::lazy_static;
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    storage: StorageConfig,
}

#[derive(Deserialize)]
struct StorageConfig {
    upload_dir: String,
}

lazy_static! {
    static ref TOKEN_STORE: RwLock<HashMap<String, OffsetDateTime>> = 
        RwLock::new(HashMap::new());
    static ref CONFIG: Config = load_config();
}

fn load_config() -> Config {
    let config_str = std::fs::read_to_string("puttr.toml")
        .expect("Failed to read puttr.toml configuration file");
    toml::from_str(&config_str)
        .expect("Failed to parse puttr.toml configuration file")
}

fn main() {
    let mut router = Router::new();

    router.get("/", index, "index");
    router.get("/token", get_token, "token");
    router.put("/data", put_data, "data");

    println!("Running at http://localhost:3000");
    Iron::new(router).http("localhost:3000").unwrap();
}


fn index(_request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    response.set_mut(status::Ok);
    response.set_mut("text/html; charset=utf-8".parse::<iron::mime::Mime>().unwrap());
    response.set_mut(r#"
        <!DOCTYPE html>
        <html>
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>📮 puttr - Secure Data Posting Service</title>
            <style>
                * { margin: 0; padding: 0; box-sizing: border-box; }
                body {
                    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
                    line-height: 1.6;
                    color: #2c3e50;
                    background: linear-gradient(135deg, #f5f7fa 0%, #c3cfe2 100%);
                    min-height: 100vh;
                    padding: 20px;
                }
                .container {
                    max-width: 800px;
                    margin: 0 auto;
                    background: white;
                    padding: 40px;
                    border-radius: 10px;
                    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
                }
                h1 {
                    color: #dc3545;
                    font-size: 2.5em;
                    margin-bottom: 10px;
                }
                .subtitle {
                    color: #7f8c8d;
                    font-size: 1.1em;
                    margin-bottom: 30px;
                }
                section {
                    margin-bottom: 35px;
                }
                h2 {
                    color: #dc3545;
                    font-size: 1.6em;
                    margin-bottom: 15px;
                    border-bottom: 2px solid #dc3545;
                    padding-bottom: 10px;
                }
                p {
                    margin-bottom: 12px;
                    color: #555;
                }
                code {
                    background: #f0f0f0;
                    padding: 2px 6px;
                    border-radius: 3px;
                    color: #dc3545;
                    font-family: monospace;
                }
                pre {
                    background: #f8f9fa;
                    border-left: 4px solid #dc3545;
                    padding: 15px;
                    border-radius: 6px;
                    margin: 15px 0;
                    font-family: monospace;
                    font-size: 0.95em;
                    color: #2c3e50;
                    overflow-x: auto;
                }
                .step {
                    background: #f8f9fa;
                    padding: 20px;
                    border-left: 4px solid #2196f3;
                    border-radius: 6px;
                    margin: 20px 0;
                }
                .step h3 {
                    color: #2196f3;
                    margin-bottom: 10px;
                }
                .highlight {
                    background: #fff3cd;
                    border-left: 4px solid #ffc107;
                    padding: 15px;
                    border-radius: 6px;
                    margin: 20px 0;
                    color: #856404;
                }
                .link {
                    color: #dc3545;
                    text-decoration: none;
                    font-weight: 600;
                }
                .link:hover {
                    text-decoration: underline;
                }
                ul {
                    margin-left: 20px;
                    margin: 15px 0;
                }
                li {
                    margin: 8px 0;
                    color: #555;
                }
            </style>
        </head>
        <body>
            <div class="container">
                <h1>📮 puttr</h1>
                <p class="subtitle">A secure, lightweight web service for posting and storing data with token authentication.</p>

                <section>
                    <h2>Quick Start</h2>
                    <p>Follow these three simple steps to send data securely to puttr:</p>
                </section>

                <section class="step">
                    <h3>Step 1: Request an Authentication Token</h3>
                    <p>First, get a token that's valid for 5 minutes. Choose one of these commands:</p>
                    <p><strong>Using curl:</strong></p>
                    <pre>curl http://localhost:3000/token</pre>
                    <p><strong>Using httpie:</strong></p>
                    <pre>http GET http://localhost:3000/token</pre>
                    <p>The response will be a base64-encoded token. Save it for the next step.</p>
                </section>

                <section class="step">
                    <h3>Step 2: Send Data with Your Token</h3>
                    <p>Use your token to send data via a PUT request. Include it in the <code>Authorization: Token &lt;your-token&gt;</code> header. The <code>Content-Type</code> header determines the file extension.</p>
                    <p><strong>Using curl (JSON):</strong></p>
                    <pre>curl -X PUT \
  -H "Content-Type: application/json" \
  -H "Authorization: Token YOUR_TOKEN_HERE" \
  -d "content={\"key\": \"value\"}" \
  http://localhost:3000/data</pre>
                    <p><strong>Using curl (Plain Text):</strong></p>
                    <pre>curl -X PUT \
  -H "Content-Type: text/plain" \
  -H "Authorization: Token YOUR_TOKEN_HERE" \
  -d "content=hello world" \
  http://localhost:3000/data</pre>
                    <p>Replace <code>YOUR_TOKEN_HERE</code> with the token from Step 1, and update the <code>Content-Type</code> and content as needed.</p>
                </section>

                <section class="step">
                    <h3>Step 3: Done!</h3>
                    <p>Your data has been stored securely in a timestamped file. The file extension is determined by the Content-Type header you sent. Each upload gets its own file with a unique name combining the token and current timestamp.</p>
                </section>

                <section>
                    <h2>Important Information</h2>
                    <ul>
                        <li><strong>Token Validity:</strong> Tokens expire after 5 minutes. Request a new one if needed.</li>
                        <li><strong>Content Field:</strong> The <code>content</code> field is required and must not be empty.</li>
                        <li><strong>Authorization Required:</strong> All PUT requests to <code>/data</code> require a valid token.</li>
                        <li><strong>File Extensions:</strong> Files are saved with extensions based on the <code>Content-Type</code> header (e.g., .json, .xml, .txt).</li>
                        <li><strong>Storage:</strong> Data is stored in <code>uploads/YYYY-MM/data-&lt;timestamp&gt;-&lt;token&gt;.&lt;ext&gt;</code></li>
                    </ul>
                </section>

                <section>
                    <h2>Full Example</h2>
                    <p>Here's a complete example that retrieves a token and sends JSON data in one script:</p>
                    <pre>#!/bin/bash
TOKEN=$(curl -s http://localhost:3000/token)
curl -X PUT \
  -H "Content-Type: application/json" \
  -H "Authorization: Token $TOKEN" \
  -d "content={\"message\": \"Hello from puttr\"}" \
  http://localhost:3000/data</pre>
                </section>

                <section class="highlight">
                    <strong>📖 Full Documentation:</strong> For complete documentation, visit <a href="docs/index.html" class="link">the landing page</a> or check the README.md file.
                </section>
            </div>
        </body>
        </html>
    "#);

    println!("GET / 200");
    Ok(response)
}


fn get_token(_request: &mut Request) -> IronResult<Response> {
    let token = generate_token();
    let expiration = OffsetDateTime::now_utc() + Duration::minutes(5);

    {
        let mut store = TOKEN_STORE.write()
            .expect("Failed to acquire write lock");
        store.insert(token.clone(), expiration);
        clean_expired_tokens(&mut store);
    }

    println!("GET /token 200 - Token generated");
    Ok(Response::with((status::Ok, token)))
}


fn generate_token() -> String {
    let uuid = Uuid::new_v4();
    base64::engine::general_purpose::STANDARD.encode(uuid.as_bytes())
}


fn clean_expired_tokens(store: &mut HashMap<String, OffsetDateTime>) {
    let now = OffsetDateTime::now_utc();
    store.retain(|_token, expiration| *expiration > now);
}


fn put_data(request: &mut Request) -> IronResult<Response> {
    use params::{Params, Value};

    let token = extract_token_from_header(request);

    let token_value = match token {
        None => {
            println!("PUT /data 401 - Missing authorization token");
            return Ok(Response::with(status::Unauthorized));
        },
        Some(tv) => {
            let store = TOKEN_STORE.read()
                .expect("Failed to acquire read lock");
            
            if !store.contains_key(&tv) {
                println!("PUT /data 401 - Invalid or expired token");
                return Ok(Response::with(status::Unauthorized));
            }
            tv
        }
    };

    let content_type = extract_content_type(request);
    let map = request.get_ref::<Params>().unwrap();

    match map.find(&["content"]) {
        Some(&Value::String(ref name)) if name.len() > 0 => {
            println!("PUT /data ({} bytes, {})", name.len(), name);

            let file_extension = content_type_to_extension(&content_type);
            let file_path = generate_file_path(&token_value, &file_extension);
            
            if let Some(parent) = file_path.parent() {
                if let Err(why) = create_dir_all(parent) {
                    panic!("couldn't create directories: {}", why);
                }
            }

            let mut file = match File::create(&file_path) {
                Err(why) => panic!("couldn't create file: {}", why),
                Ok(file) => file,
            };
            match file.write_all(name.as_bytes()) {
                Err(why) => panic!("couldn't write data: {}", why),
                Ok(_) => println!("data written to file: {}", file_path.display()),
            };

            Ok(Response::with((status::Ok, "success")))
        },
        _ => Ok(Response::with(status::NotFound)),
    }
}


fn generate_file_path(token: &str, extension: &str) -> PathBuf {
    let now = OffsetDateTime::now_utc();
    let timestamp = now.format_iso8601_timestamp();
    let year_month = now.format_year_month();
    
    PathBuf::from(format!(
        "{}/{}/data-{}-{}.{}",
        CONFIG.storage.upload_dir, year_month, timestamp, token, extension
    ))
}


fn extract_token_from_header(request: &Request) -> Option<String> {
    request.headers.get_raw("Authorization")
        .and_then(|values| values.first())
        .and_then(|value| {
            let value_str = String::from_utf8(value.clone()).ok()?;
            if value_str.starts_with("Token ") {
                Some(value_str[6..].to_string())
            } else {
                None
            }
        })
}


fn extract_content_type(request: &Request) -> String {
    request.headers.get_raw("Content-Type")
        .and_then(|values| values.first())
        .and_then(|value| String::from_utf8(value.clone()).ok())
        .unwrap_or_else(|| "text/plain".to_string())
}


fn content_type_to_extension(content_type: &str) -> String {
    let ct = content_type.trim().to_lowercase();
    let base_type = ct.split(';').next().unwrap_or(&ct).trim();

    match base_type {
        // Application types
        "application/json" => "json",
        "application/xml" => "xml",
        "application/pdf" => "pdf",
        "application/zip" => "zip",
        "application/gzip" => "gz",
        "application/x-gzip" => "gz",
        "application/x-tar" => "tar",
        "application/x-rar-compressed" => "rar",
        "application/x-7z-compressed" => "7z",
        "application/x-bzip2" => "bz2",
        "application/octet-stream" => "bin",
        "application/x-www-form-urlencoded" => "form",
        
        // Text types
        "text/plain" => "txt",
        "text/html" => "html",
        "text/css" => "css",
        "text/javascript" => "js",
        "text/csv" => "csv",
        "text/yaml" => "yaml",
        "text/x-yaml" => "yaml",
        "text/markdown" => "md",
        "text/x-markdown" => "md",
        
        // Image types
        "image/png" => "png",
        "image/jpeg" => "jpg",
        "image/jpg" => "jpg",
        "image/gif" => "gif",
        "image/webp" => "webp",
        "image/svg+xml" => "svg",
        "image/bmp" => "bmp",
        "image/tiff" => "tiff",
        "image/x-icon" => "ico",
        
        // Audio types
        "audio/mpeg" => "mp3",
        "audio/wav" => "wav",
        "audio/webm" => "webm",
        "audio/flac" => "flac",
        "audio/ogg" => "ogg",
        "audio/aac" => "aac",
        
        // Video types
        "video/mp4" => "mp4",
        "video/webm" => "webm",
        "video/mpeg" => "mpeg",
        "video/quicktime" => "mov",
        "video/x-msvideo" => "avi",
        "video/x-matroska" => "mkv",
        "video/x-flv" => "flv",
        
        // Default to txt for unknown types
        _ => "txt",
    }.to_string()
}


trait DateTimeFormatting {
    fn format_iso8601_timestamp(&self) -> String;
    fn format_year_month(&self) -> String;
}

impl DateTimeFormatting for OffsetDateTime {
    fn format_iso8601_timestamp(&self) -> String {
        self.format(&time::format_description::well_known::Iso8601::DEFAULT)
            .unwrap_or_else(|_| "unknown".to_string())
    }
    
    fn format_year_month(&self) -> String {
        format!("{}-{:02}", self.year(), self.month() as u8)
    }
}
