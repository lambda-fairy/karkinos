#![feature(plugin, proc_macro)]
#![plugin(maud_macros)]

extern crate iron;
extern crate maud;
extern crate router;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use iron::prelude::*;
use iron::status;
use router::Router;

mod user;
mod views;

use user::User;

fn main() {
    let mut router = Router::new();
    router.get("/", home, "home");
    router.get("/user/:id", user, "user");
    router.get("*", not_found, "not_found");

    fn home(r: &mut Request) -> IronResult<Response> {
        let body = views::home(r);
        Ok(Response::with((status::Ok, body)))
    }

    fn user(r: &mut Request) -> IronResult<Response> {
        let route = r.extensions.get::<Router>().unwrap();
        let id = route.find("id").unwrap();
        match User::lookup(id) {
            Ok(user) => {
                let body = views::user(r, id, &user);
                Ok(Response::with((status::Ok, body)))
            }
            Err(_) => {
                let body = views::user_not_found(r, id);
                Ok(Response::with((status::NotFound, body)))
            },
        }
    }

    fn not_found(r: &mut Request) -> IronResult<Response> {
        let body = views::not_found(r);
        Ok(Response::with((status::NotFound, body)))
    }

    println!("Starting on port 8344...");
    Iron::new(router).http("localhost:8344").unwrap();
}
