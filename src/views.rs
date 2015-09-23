use iron::response::WriteBody;

use iron_maud::Maud;

pub fn default(title: String, body: String) -> Box<WriteBody + Send> {
    Maud::new(move |w| html!(*w, {
        $$"<!DOCTYPE html>"
        html {
            head {
                title $title
                style $$ r#"html { font-family: "Comic Sans MS" }"#
            }
            body {
                h1 $title
                $$body
            }
        }
    }))
}

pub fn home() -> String {
    let mut s = String::new();
    html!(s, {
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
    }).unwrap();
    s
}

pub fn not_found(url: &str) -> String {
    let mut s = String::new();
    html!(s, {
        p {
            "The page at "
            code $url
            " could not be found."
        }
        p a href="/" "<< Back to home page"
    }).unwrap();
    s
}
