#![feature(collections_bound, btree_range)]
#![feature(plugin, proc_macro)]
#![plugin(maud_macros)]

extern crate caseless;
extern crate env_logger;
extern crate iron;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate logger;
extern crate maud;
extern crate persistent;
extern crate pulldown_cmark;
#[macro_use]
extern crate router;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate staticfile;
extern crate unicode_normalization;
extern crate unicode_segmentation;
extern crate urlencoded;

use iron::prelude::*;
use iron::status;
use iron::typemap::Key;
use logger::Logger;
use router::Router;
use persistent::State;
use staticfile::Static;
use std::env;
use std::sync::{Arc, RwLock};
use urlencoded::UrlEncodedQuery;

mod models;
mod search;
mod views;

use models::Users;

lazy_static! {
    static ref IS_PRODUCTION: bool = {
        let p = env::var("KARKINOS_ENVIRONMENT")
            .map(|s| s.to_lowercase() == "production")
            .unwrap_or(false);
        if p {
            info!("running in production environment");
        } else {
            info!("running in development environment");
        }
        p
    };
}

#[derive(Copy, Clone)]
struct UsersKey;
impl Key for UsersKey { type Value = Users; }

fn main() {
    env_logger::init().unwrap();

    let mut router = Router::new();
    router.get("/", home, "home");
    router.get("/user/:id", user, "user");
    router.get("/search", search, "search");
    router.get("/static/:path", Static::new("."), "static");
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
            Some(Ok(user)) => {
                let body = views::user(r, id, &user);
                Ok(Response::with((status::Ok, body)))
            },
            Some(Err(error)) => {
                let body = views::user_error(r, id, error);
                Ok(Response::with((status::Ok, body)))
            }
            None => {
                let body = views::user_not_found(r, id);
                Ok(Response::with((status::NotFound, body)))
            },
        }
    }

    fn search(r: &mut Request) -> IronResult<Response> {
        let q: Option<String> = r.get_ref::<UrlEncodedQuery>().ok()
            .and_then(|query| query.get("q"))
            .and_then(|q| q.first().cloned());
        let users = r.extensions.get::<State<UsersKey>>().unwrap();
        let users = users.read().unwrap();
        if let Some(q) = q {
            let results = users.search(&q);
            let results = results.into_iter()
                // Restrict search to 20 results, so the server isn't bogged
                // down too much
                .take(20)
                .map(|(id, weight)| (users.get(&id).unwrap(), id, weight));
            let body = views::search_results(r, &q, results);
            Ok(Response::with((status::Ok, body)))
        } else {
            let body = views::search(r);
            Ok(Response::with((status::Ok, body)))
        }
    }

    fn not_found(r: &mut Request) -> IronResult<Response> {
        let body = views::not_found(r);
        Ok(Response::with((status::NotFound, body)))
    }

    let mut chain = Chain::new(router);

    chain.link(Logger::new(None));

    chain.link({
        let users = Users::load("rustaceans.org/data").unwrap();
        let arc = Arc::new(RwLock::new(users));
        State::<UsersKey>::both(arc)
    });

    if *IS_PRODUCTION {
        chain.link_before(|r: &mut Request| {
            // Since we're running behind a reverse proxy, the headers are kind
            // of messed up
            r.headers.set(iron::headers::Host {
                hostname: "karkinos.lambda.xyz".to_string(),
                port: None,
            });
            let mut url = r.url.clone().into_generic_url();
            url.set_scheme("https").unwrap();
            url.set_host(Some("karkinos.lambda.xyz")).unwrap();
            url.set_port(None).unwrap();
            r.url = iron::Url::from_generic_url(url).unwrap();
            Ok(())
        });
    }

    let bind_addr = "localhost:8344";
    info!("starting on {}", bind_addr);
    Iron::new(chain).http(bind_addr).unwrap();
}
