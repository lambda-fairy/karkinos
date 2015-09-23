use std::fmt;

use iron::response::WriteBody;

use iron_maud::{Maud, Template};

pub fn default(title: String, body: Template) -> Box<WriteBody + Send> {
    let mut cell = Some(body);
    Maud::new(move |w| html!(*w, {
        $$"<!DOCTYPE html>"
        html {
            head {
                title $title
                style $$ r#"html { font-family: "Comic Sans MS" }"#
            }
            body {
                h1 $title
                #call_box cell.take().unwrap()
            }
        }
    }))
}

pub fn home() -> Template {
    Box::new(move |w: &mut fmt::Write| html!(*w, {
        p {
            b "Karkinos"
            " is a list of people interested in the "
            a href="https://www.rust-lang.org" "Rust programming language"
            "."
        }
        p {
            "It uses the same data as "
            a href="http://rustaceans.org" "rustaceans.org"
            ", but with a different interface."
        }
    }))
}

pub fn not_found(url: String) -> Template {
    Box::new(move |w: &mut fmt::Write| html!(*w, {
        p {
            "The page at "
            code $url
            " could not be found."
        }
        p a href="/" "<< Back to home page"
    }))
}
