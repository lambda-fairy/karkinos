#![feature(fnbox, plugin)]
#![plugin(maud_macros)]

extern crate iron;
extern crate maud;
#[macro_use] extern crate mime;
extern crate router;
extern crate serde_json;

use iron::prelude::*;
use iron::status;
use mime::Mime;
use router::Router;

mod iron_maud;
mod user;
mod views;

fn main() {
    let mut router = Router::new();
    router.get("/", home);
    router.get("*", not_found);

    fn home(_: &mut Request) -> IronResult<Response> {
        let result = views::default(
            "Karkinos".to_owned(),
            views::home());
        Ok(Response::with((status::Ok, html_mime(), result)))
    }

    fn not_found(r: &mut Request) -> IronResult<Response> {
        // FIXME: use path
        let result = views::default(
            "Not Found".to_owned(),
            views::not_found(r.url.to_string()));
        Ok(Response::with((status::NotFound, html_mime(), result)))
    }

    println!("Starting on port 8344...");
    Iron::new(router).http("localhost:8344").unwrap();
}

fn html_mime() -> Mime {
    mime!(Text/Html; Charset=Utf8)
}
