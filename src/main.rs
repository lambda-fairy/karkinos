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
extern crate notify;
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
use notify::{DebouncedEvent, RecursiveMode, Watcher};
use router::Router;
use persistent::State;
use staticfile::Static;
use std::env;
use std::sync::{Arc, RwLock};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use urlencoded::UrlEncodedQuery;

mod models;
mod search;
mod views;

use models::Users;

const DATA_PATH: &'static str = "rustaceans.org/data";

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
        fn get_query<'r>(r: &'r mut Request, key: &str) -> Option<&'r str> {
            r.get_ref::<UrlEncodedQuery>().ok()
                .and_then(|query| query.get(key))
                .and_then(|q| q.first().map(AsRef::as_ref))
        }
        let q = get_query(r, "q").map(ToString::to_string);
        let is_raw = get_query(r, "raw").and_then(|s| s.parse().ok());
        let users = r.extensions.get::<State<UsersKey>>().unwrap();
        let users = users.read().unwrap();
        if let Some(q) = q {
            let results = users.search(&q);
            let results = results.into_iter()
                // Restrict search to 20 results, so the server isn't bogged
                // down too much
                .take(20)
                .map(|(id, weight)| (users.get(&id).unwrap(), id, weight));
            let body = if is_raw == Some(true) {
                views::search_results_raw(r, results)
            } else {
                views::search_results(r, &q, results)
            };
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
        // Load user data
        let users = Users::load(DATA_PATH).unwrap();
        let arc = Arc::new(RwLock::new(users));
        // Reload data automatically when changed
        let arc_cloned = arc.clone();
        thread::spawn(move || {
            let (tx, rx) = mpsc::channel();
            let mut watcher = notify::watcher(tx, Duration::from_secs(10)).unwrap();
            watcher.watch(DATA_PATH, RecursiveMode::NonRecursive).unwrap();
            loop {
                match rx.recv() {
                    Ok(DebouncedEvent::NoticeWrite(..)) |
                    Ok(DebouncedEvent::NoticeRemove(..)) |
                    Ok(DebouncedEvent::Chmod(..)) => continue,  // Ignore trivial events
                    Ok(DebouncedEvent::Error(e, _path)) => error!("watch error: {}", e),
                    Ok(_) => info!("received file system update; reloading"),
                    Err(e) => error!("watch error: {}", e),
                }
                let users = Users::load(DATA_PATH).unwrap();
                *arc_cloned.write().unwrap() = users;
            }
        });
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
