use iron::prelude::*;
use iron::status;
use router::Router;
use std::io::prelude::*;
use std::fs::File;


fn main() {
    let mut router = Router::new();

    router.get("/", index, "index");
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


fn put_data(_request: &mut Request) -> IronResult<Response> {
    use params::{Params, Value};

    let map = _request.get_ref::<Params>().unwrap();

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
