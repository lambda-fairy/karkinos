use iron::prelude::*;
use maud::{Markup, PreEscaped};

use user::User;

static STYLES: &'static str = r#"
html {
    box-sizing: border-box;
    font: 100%/1.5 "Liberation Serif", "Times New Roman", serif;
    background: linear-gradient(to bottom, #445 0%, #001 100%);
}

* {
    box-sizing: inherit;
}

body {
    margin: 0 auto;
    max-width: 50rem;
    min-height: 100vh;
    padding: 0.5rem 2rem;
    box-shadow: 0 0 0 8px #ccc, 0 0 0 16px #999, 0 0 0 20px #666;
    background: #fff;
}

a {
    text-decoration: none;
    color: #910;
}

a:hover, a:active {
	text-shadow: 0 0 2px #fff, 0 0 8px #d60;
}

h1 {
    margin: 0 0 1rem;
    font-family: "Comic Sans MS", sans-serif;
    font-size: 5rem;
    letter-spacing: 1.25rem;
}

h1 a {
    color: #f90;
    text-shadow: 2px 2px #000, -2px -2px #faa;
    text-transform: uppercase;
}

h1 a:hover, h1 a:active {
    text-shadow: 8px 8px #000, -8px -8px #faa;
}

table {
    border-spacing: 0.5rem;
}

th {
    min-width: 7rem;
    text-align: right;
    vertical-align: top;
}
"#;

pub fn layout(r: &Request, title: Option<&str>, body: Markup) -> Markup {
    let url = r.url.clone().into_generic_url();
    html! {
        (PreEscaped("<!DOCTYPE html>"))
        html {
            meta charset="utf-8" /
            title {
                @if let Some(title) = title {
                    (title) " - "
                }
                "Karkinos"
            }
            style (PreEscaped(STYLES))
            body {
                h1 a href="/" "🦀Karkinos"
                @if let Some(title) = title {
                    h2 a href=(url.path()) title="Link to this page" (title)
                }
                (body)
            }
        }
    }
}

pub fn home(r: &Request) -> Markup {
    layout(r, None, html! {
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

pub fn not_found(r: &Request) -> Markup {
    layout(r, Some("Not found"), html! {
        p {
            "The page at "
            strong (r.url)
            " could not be found."
        }
        p a href="/" "<< Back to home page"
    })
}

pub fn user(r: &Request, id: &str, user: &User) -> Markup {
    let title = if let Some(ref name) = user.name {
        format!("{} ({})", name, id)
    } else {
        id.to_string()
    };
    layout(r, Some(&title), html! {
        table {
            @if let Some(ref x) = user.name {
                tr {
                    th "Name"
                    td (x)
                }
            }
            @if let Some(ref x) = user.irc {
                tr {
                    th "IRC nick"
                    td (x)
                }
            }
            @if !user.irc_channels.is_empty() {
                tr {
                    th "IRC channels"
                    td @for (i, channel) in user.irc_channels.iter().enumerate() {
                        @if i > 0 { ", " }
                        "#" (channel)
                    }
                }
            }
            @if let Some(ref x) = user.email {
                tr {
                    th "Email"
                    td a href={ "mailto:" (x) } (x)
                }
            }
            @if let Some(ref x) = user.discourse {
                tr {
                    th "Discourse"
                    td a href={ "https://users.rust-lang.org/users/" (x) } (x)
                }
            }
            @if let Some(ref x) = user.reddit {
                tr {
                    th "Reddit"
                    td a href={ "https://reddit.com/user/" (x) } (x)
                }
            }
            @if let Some(ref x) = user.notes {
                tr {
                    th "Notes"
                    td style="white-space: pre-line" (x)
                }
            }
        }
    })
}

pub fn user_not_found(r: &Request, id: &str) -> Markup {
    layout(r, Some(id), html! {
        p {
            "The user "
            strong (id)
            " could not be found."
        }
        p a href="/" "<< Back to home page"
    })
}
