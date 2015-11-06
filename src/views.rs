use std::fmt;

use maud::PreEscaped;

use user::User;

static STYLES: &'static str = r#"
html {
    font-family: "Comic Sans MS", sans-serif;
}
h1 {
    color: #f90;
    font-size: 500%;
    letter-spacing: 0.75em;
    margin: 1rem 0;
    text-shadow: 2px 2px #000, -2px -2px #faa;
    text-transform: uppercase;
}
"#;

pub fn default<F>(w: &mut fmt::Write, title: &str, body: F) -> fmt::Result where
    F: FnOnce(&mut fmt::Write) -> fmt::Result
{
    html!(*w, {
        $PreEscaped("<!DOCTYPE html>")
        html {
            head {
                title $title
                style $PreEscaped(STYLES)
            }
            body {
                h1 "Karkinos"
                h2 $title
                #call body
            }
        }
    })
}

pub fn home(w: &mut fmt::Write) -> fmt::Result {
    html!(*w, {
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
    })
}

pub fn not_found(w: &mut fmt::Write, url: &str) -> fmt::Result {
    html!(*w, {
        p {
            "The page at "
            strong $url
            " could not be found."
        }
        p a href="/" "<< Back to home page"
    })
}

pub fn user(w: &mut fmt::Write, user: &User) -> fmt::Result {
    let make_row = |w: &mut fmt::Write, key: &str, value: &Option<String>|
        html!(*w, #if let Some(ref value) = *value {
            tr {
                th $key
                td $value
            }
        });
    html!(*w, {
        table {
            #call (|w| make_row(w, "Name", &user.name))
            #call (|w| make_row(w, "IRC nick", &user.irc))
            #if !user.irc_channels.is_empty() {
                tr {
                    th "IRC channels"
                    td #for (i, channel) in user.irc_channels.iter().enumerate() {
                        #if i > 0 { ", " }
                        $channel
                    }
                }
            }
            #call (|w| make_row(w, "Email", &user.email))
            #call (|w| make_row(w, "Discourse", &user.discourse))
            #call (|w| make_row(w, "Notes", &user.notes))
        }
    })
}

pub fn user_not_found(w: &mut fmt::Write, name: &str) -> fmt::Result {
    html!(*w, {
        p {
            "The user "
            strong $name
            " could not be found."
        }
        p a href="/" "<< Back to home page"
    })
}
