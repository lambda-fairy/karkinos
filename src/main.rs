#![feature(plugin, proc_macro)]
#![plugin(maud_macros)]

extern crate iron;
extern crate maud;
extern crate persistent;
extern crate router;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use iron::prelude::*;
use iron::status;
use iron::typemap::Key;
use router::Router;
use persistent::State;
use std::collections::BTreeMap;
use std::ffi::OsStr;
use std::fs;
use std::sync::{Arc, RwLock};

mod user;
mod views;

use user::User;

#[derive(Copy, Clone)]
struct UsersKey;
impl Key for UsersKey { type Value = Users; }

type Users = BTreeMap<String, User>;

fn load_users() -> Result<Users, serde_json::Error> {
    let mut users = BTreeMap::new();
    for entry in fs::read_dir("rustaceans.org/data")? {
        let path = entry?.path();
        if path.extension() == Some(OsStr::new("json")) {
            let id = path.file_stem().unwrap().to_string_lossy().into_owned();
            print!("Loading {}...", id);
            let user = User::from_path(&path)?;
            println!(" done!");
            users.insert(id, user);
        }
    }
    Ok(users)
}

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

    fn not_found(r: &mut Request) -> IronResult<Response> {
        let body = views::not_found(r);
        Ok(Response::with((status::NotFound, body)))
    }

    let mut chain = Chain::new(router);
    chain.link({
        let users = load_users().unwrap();
        let state = State::<UsersKey>::from(Arc::new(RwLock::new(users)));
        (state.clone(), state)
    });

    println!("Starting on port 8344...");
    Iron::new(chain).http("localhost:8344").unwrap();
}
