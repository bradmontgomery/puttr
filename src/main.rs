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

lazy_static! {
    static ref TOKEN_STORE: RwLock<HashMap<String, OffsetDateTime>> = 
        RwLock::new(HashMap::new());
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
        <!Doctype html>
        <html><head><title>puttr</title></head>
        <body>
            <h1>puttr</h1>
            <p>Send your form-data as a PUT request to <code>/data</code> with
            the key <code>content</code>, e.g.:</p>
            <pre>
            content: hello world
            </pre>
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

    let map = request.get_ref::<Params>().unwrap();

    match map.find(&["content"]) {
        Some(&Value::String(ref name)) if name.len() > 0 => {
            println!("PUT /data ({} bytes, {})", name.len(), name);

            let file_path = generate_file_path(&token_value);
            
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


fn generate_file_path(token: &str) -> PathBuf {
    let now = OffsetDateTime::now_utc();
    let timestamp = now.format_iso8601_timestamp();
    let year_month = now.format_year_month();
    
    PathBuf::from(format!(
        "uploads/{}/data-{}-{}.txt",
        year_month, timestamp, token
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
