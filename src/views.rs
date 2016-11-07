use iron::prelude::*;
use maud::{Markup, PreEscaped, Render};
use pulldown_cmark;

use models::User;

static STYLES: &'static str = r#"
html {
    box-sizing: border-box;
    font: 100%/1.5 "Liberation Serif", "Times New Roman", serif;
    background: linear-gradient(to bottom, #445 0%, #001 100%);
}

@media (min-width: 450px) {
    html {
        font-size: 125%;
    }
}

@media (min-width: 750px) {
    html {
        font-size: 150%;
    }
}

input, button {
    font-size: 1rem;
    line-height: 1.5;
}

* {
    box-sizing: inherit;
}

body {
    margin: 0 auto;
    max-width: 30rem;
    min-height: 100vh;
    padding: 0.5rem 1.5rem;
    box-shadow: 0 0 0 8px #ccc, 0 0 0 16px #999, 0 0 0 20px #666;
    background: #fff;
}

a {
    text-decoration: none;
    color: #910;
}

a:hover, a:focus, a:active {
    text-shadow: 0 0 2px #fff, 0 0 8px #d60;
}

h1 {
    /* A E S T H E T I C */
    margin: 0 0 1rem;
    white-space: nowrap;
    font-family: "Comic Sans MS", sans-serif;
    font-size: 1.75rem;
    letter-spacing: 0.5rem;
}

@media (min-width: 600px) {
    h1 {
        font-size: 3rem;
        letter-spacing: 0.75rem;
    }
}

h1 a {
    color: #f90;
    text-transform: uppercase;
    text-shadow: 2px 2px #000, -2px -2px #faa;
    transition: text-shadow 0.2s;
}

h1 a:hover, h1 a:focus, h1 a:active {
    text-shadow: 16px 16px #000, -16px -16px #faa;
    transition: text-shadow 0.2s;
}

.karkinos {
    font-family: "Comic Sans MS", sans-serif;
}

#q {
    width: 100%;
    padding: 0.25rem 0.5rem;
    border: 1px solid #000;
    box-shadow: 0 0 0 0 #f90, 0 0 0 0 #666;
    transition: box-shadow 0.2s;
}

#q:focus {
    border-color: #f90;
    box-shadow: -2px -2px 0 2px #f90, 2px 2px 0 3px #666;
    transition: box-shadow 0.2s;
}

table {
    border-spacing: 0.5rem;
}

th {
    text-align: right;
    vertical-align: top;
}
"#;

pub fn layout(r: &Request, title: Option<&str>, body: Markup) -> Markup {
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
            meta name="viewport" content="width=device-width" /
            style (PreEscaped(STYLES))
            body {
                h1 a href="/" "🦀Karkinos"
                @if let Some(title) = title {
                    h2 a href=(r.url) title="Link to this page" (title)
                }
                (body)
            }
        }
    }
}

pub fn home(r: &Request) -> Markup {
    layout(r, None, html! {
        (search_form(r))
        p {
            span.karkinos "KARKINOS"
            " is a database of people interested in the "
            a href="https://www.rust-lang.org" "Rust programming language"
            ". It uses the same data as "
            a href="http://rustaceans.org" "rustaceans.org"
            ", but presents it through a different interface."
        }
        p "I created Karkinos for these reasons:"
        ul {
            li "To provide access for users who browse with JavaScript disabled;"
            li "To rewrite the backend in Rust (instead of Node.js);"
            li {
                "As a proving ground for my template engine, "
                a href="https://github.com/lfairy/maud" "Maud"
                ";"
            }
            li "To screw around with CSS (this is the most important reason)."
        }
        p {
            "Karkinos is named after a very special "
            a href="https://en.wikipedia.org/wiki/Cancer_(constellation)#Names" "giant crab"
            "."
        }
        p {
            "The source code for this site can be found on "
            a href="https://github.com/lfairy/karkinos" "GitHub"
            "."
        }
    })
}

fn search_form(r: &Request) -> Markup {
    html! {
        form action=(url_for!(r, "search")) {
            input name="q" id="q" type="search" placeholder="Search" /
        }
        script (PreEscaped("document.getElementById('q').select()"))
    }
}

pub fn not_found(r: &Request) -> Markup {
    layout(r, Some("Not found"), html! {
        p {
            "The page at "
            strong (r.url)
            " could not be found."
        }
        p a href="/" "‹ Back to home page"
    })
}

pub fn search(r: &Request) -> Markup {
    layout(r, Some("Search"), html! {
        (search_form(r))
    })
}

pub fn search_results(r: &Request, query: &str, results: &[(String, u64)]) -> Markup {
    let title = format!("Search results for “{}”", query);
    layout(r, Some(&title), html! {
        @for &(ref id, weight) in results {
            p title={ "Weight: " (weight) } {
                a href=(url_for!(r, "user", "id" => id[..])) (id)
            }
        }
        @if results.is_empty() {
            p {
                "No results found. "
                a href=(url_for!(r, "home")) "Try another query?"
            }
        }
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
            @if let Some(ref nick) = user.irc {
                tr {
                    th "IRC"
                    td {
                        (nick)
                        @if !user.irc_channels.is_empty() {
                            " on "
                            @for (i, channel) in user.irc_channels.iter().enumerate() {
                                @if i > 0 { ", " }
                                a href={ "irc://irc.mozilla.org/" (channel) } {
                                    "#" (channel)
                                }
                            }
                        }
                    }
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
            @if let Some(ref x) = user.twitter {
                tr {
                    th "Twitter"
                    td a href={ "https://twitter.com/" (x) } (x)
                }
            }
            @if let Some(ref x) = user.website {
                tr {
                    th "Website"
                    td a href=(x) (x)
                }
            }
            @if let Some(ref x) = user.blog {
                tr {
                    th "Blog"
                    td a href=(x) (x)
                }
            }
            @if let Some(ref x) = user.email {
                tr {
                    th "Email"
                    td a href={ "mailto:" (x) } (x)
                }
            }
        }
        @if let Some(ref x) = user.notes {
            div.notes (Markdown(x))
        }
    })
}

pub fn user_error(r: &Request, id: &str, error: &str) -> Markup {
    layout(r, Some(id), html! {
        p {
            "The user "
            strong (id)
            " exists, but their entry could not be parsed."
        }
        p {
            "(Error: " (error) ")"
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
        p a href="/" "‹ Back to home page"
    })
}

struct Markdown<'a>(&'a str);

impl<'a> Render for Markdown<'a> {
    fn render_to(&self, buffer: &mut String) {
        let parser = pulldown_cmark::Parser::new(self.0);
        pulldown_cmark::html::push_html(buffer, parser);
    }
}
