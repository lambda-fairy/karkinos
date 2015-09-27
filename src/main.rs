#![feature(plugin)]
#![plugin(maud_macros)]

extern crate iron;
extern crate iron_maud;
extern crate maud;
extern crate router;
extern crate serde;
extern crate serde_json;

use iron::prelude::*;
use iron::status;
use router::Router;

mod user;
mod views;

use user::User;

fn main() {
    let mut router = Router::new();
    router.get("/", home);
    router.get("/user/:name", user);
    router.get("*", not_found);

    fn home(_: &mut Request) -> IronResult<Response> {
        let result = views::default(
            "Karkinos".to_owned(),
            views::home());
        Ok(Response::with((status::Ok, result)))
    }

    fn user(r: &mut Request) -> IronResult<Response> {
        let route = r.extensions.get::<Router>().unwrap();
        let name = route.find("name").unwrap();
        match User::lookup(name) {
            Ok(user) => {
                let result = views::default(
                    name.to_owned(),
                    views::user(user));
                Ok(Response::with((status::Ok, result)))
            }
            Err(_) => {
                let result = views::default(
                    name.to_owned(),
                    views::user_not_found(name.to_owned()));
                Ok(Response::with((status::NotFound, result)))
            },
        }
    }

    fn not_found(r: &mut Request) -> IronResult<Response> {
        // FIXME: use path
        let result = views::default(
            "Not Found".to_owned(),
            views::not_found(r.url.to_string()));
        Ok(Response::with((status::NotFound, result)))
    }

    println!("Starting on port 8344...");
    Iron::new(router).http("localhost:8344").unwrap();
}
