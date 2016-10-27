#![feature(plugin, proc_macro)]
#![plugin(maud_macros)]

extern crate iron;
extern crate maud;
extern crate persistent;
extern crate pulldown_cmark;
#[macro_use]
extern crate router;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate urlencoded;

use iron::modifiers::Redirect;
use iron::prelude::*;
use iron::status;
use iron::typemap::Key;
use router::Router;
use persistent::State;
use std::sync::{Arc, RwLock};
use urlencoded::UrlEncodedQuery;

mod user;
mod views;

use user::Users;

#[derive(Copy, Clone)]
struct UsersKey;
impl Key for UsersKey { type Value = Users; }

fn main() {
    let mut router = Router::new();
    router.get("/", home, "home");
    router.get("/user/:id", user, "user");
    router.get("/search", search, "search");
    router.get("*", not_found, "not_found");

    fn home(r: &mut Request) -> IronResult<Response> {
        let body = views::home(r);
        Ok(Response::with((status::Ok, body)))
    }

    fn user(r: &mut Request) -> IronResult<Response> {
        let route = r.extensions.get::<Router>().unwrap();
        let id = route.find("id").unwrap();
        let users = r.extensions.get::<State<UsersKey>>().unwrap();
        match users.read().unwrap().get(id) {
            Some(user) => {
                let body = views::user(r, id, &user);
                Ok(Response::with((status::Ok, body)))
            }
            None => {
                let body = views::user_not_found(r, id);
                Ok(Response::with((status::NotFound, body)))
            },
        }
    }

    fn search(r: &mut Request) -> IronResult<Response> {
        let id: Option<String> = r.get_ref::<UrlEncodedQuery>().ok()
            .and_then(|query| query.get("q"))
            .and_then(|q| q.first().cloned());
        if let Some(id) = id {
            let users = r.extensions.get::<State<UsersKey>>().unwrap();
            let users = users.read().unwrap();
            if users.get(&id).is_some() {
                let url = url_for!(r, "user", "id" => id);
                Ok(Response::with((status::Found, Redirect(url))))
            } else {
                let body = views::user_not_found(r, &id);
                Ok(Response::with((status::NotFound, body)))
            }
        } else {
            not_found(r)
        }
    }

    fn not_found(r: &mut Request) -> IronResult<Response> {
        let body = views::not_found(r);
        Ok(Response::with((status::NotFound, body)))
    }

    let mut chain = Chain::new(router);
    chain.link({
        let users = Users::load("rustaceans.org/data").unwrap();
        let state = State::<UsersKey>::from(Arc::new(RwLock::new(users)));
        (state.clone(), state)
    });

    println!("Starting on localhost:8344...");
    Iron::new(chain).http("localhost:8344").unwrap();
}
