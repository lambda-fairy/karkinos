#![feature(plugin)]
#![plugin(maud_macros)]

extern crate iron;
extern crate maud;
#[macro_use] extern crate mime;
extern crate router;
extern crate serde;
extern crate serde_json;

use iron::prelude::*;
use iron::status;
use mime::Mime;
use router::Router;
use std::fmt;

mod user;
mod views;

use user::User;

fn main() {
    let mut router = Router::new();
    router.get("/", home);
    router.get("/user/:name", user);
    router.get("*", not_found);

    fn home(_: &mut Request) -> IronResult<Response> {
        let result = make_response(
            |w| views::default(w, "Karkinos",
                |w| views::home(w)));
        Ok(Response::with((status::Ok, result)))
    }

    fn user(r: &mut Request) -> IronResult<Response> {
        let route = r.extensions.get::<Router>().unwrap();
        let name = route.find("name").unwrap();
        match User::lookup(name) {
            Ok(user) => {
                let result = make_response(
                    |w| views::default(w, name,
                        |w| views::user(w, &user)));
                Ok(Response::with((status::Ok, result)))
            }
            Err(_) => {
                let result = make_response(
                    |w| views::default(w, name,
                        |w| views::user_not_found(w, name)));
                Ok(Response::with((status::NotFound, result)))
            },
        }
    }

    fn not_found(r: &mut Request) -> IronResult<Response> {
        // FIXME: use path
        let result = make_response(
            |w| views::default(w, "Not Found",
                |w| views::not_found(w, &r.url.to_string())));
        Ok(Response::with((status::NotFound, result)))
    }

    println!("Starting on port 8344...");
    Iron::new(router).http("localhost:8344").unwrap();
}

fn make_response<F>(callback: F) -> (Mime, String) where
    F: FnOnce(&mut fmt::Write) -> fmt::Result
{
    let mime = mime!(Text/Html; Charset=Utf8);
    let mut s = String::new();
    callback(&mut s).unwrap();
    (mime, s)
}
