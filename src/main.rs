use iron::prelude::*;
use iron::status;
use router::Router;
use std::io::prelude::*;
use std::fs::File;
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

    match token {
        None => {
            println!("PUT /data 401 - Missing authorization token");
            return Ok(Response::with(status::Unauthorized));
        },
        Some(token_value) => {
            let store = TOKEN_STORE.read()
                .expect("Failed to acquire read lock");
            
            if !store.contains_key(&token_value) {
                println!("PUT /data 401 - Invalid or expired token");
                return Ok(Response::with(status::Unauthorized));
            }
        }
    }

    let map = request.get_ref::<Params>().unwrap();

    match map.find(&["content"]) {
        Some(&Value::String(ref name)) if name.len() > 0 => {
            println!("PUT /data ({} bytes, {})", name.len(), name);

            let mut file = match File::create("data.txt") {
                Err(why) => panic!("couldn't create data.txt: {}", why),
                Ok(file) => file,
            };
            match file.write_all(name.as_bytes()) {
                Err(why) => panic!("couldn't write data: {}", why),
                Ok(_) => println!("data written to file"),
            };

            Ok(Response::with((status::Ok, "success")))
        },
        _ => Ok(Response::with(status::NotFound)),
    }
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
