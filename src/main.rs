extern crate iron;
extern crate serde_json;

use iron::prelude::*;
use iron::status;

mod user;

fn main() {
    fn hello_world(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Hello, world!")))
    }

    println!("Starting up...");
    Iron::new(hello_world).http("localhost:3000").unwrap();
}
