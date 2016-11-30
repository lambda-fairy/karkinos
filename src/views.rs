use ammonia;
use iron::prelude::*;
use maud::{Markup, PreEscaped, Render};
use pulldown_cmark::{self, Event, Parser, Tag};

use models::User;

fn layout(r: &Request, title: Option<&str>, body: Markup) -> Markup {
    layout_inner(r, title, title, body)
}

fn layout_inner(r: &Request, head_title: Option<&str>, body_title: Option<&str>, body: Markup) -> Markup {
    html! {
        (PreEscaped("<!DOCTYPE html>"))
        html {
            meta charset="utf-8" /
            title {
                @if let Some(head_title) = head_title {
                    (head_title) " - "
                }
                "Karkinos"
            }
            meta name="viewport" content="width=device-width" /
            link rel="stylesheet" href=(url_for!(r, "static", "path" => "styles.css")) /
            link rel="icon" type="image/png" href=(url_for!(r, "static", "path" => "icon.png")) /
            body {
                h1 a href="/" {
                    span.thecrab "🦀"
                    "Karkinos"
                }
                @if let Some(body_title) = body_title {
                    h2 a href=(r.url) title="Link to this page" (body_title)
                }
                (body)
            }
        }
    }
}

pub fn home(r: &Request) -> Markup {
    layout(r, None, html! {
        (search_form(r, ""))
        p {
            "… or view a "
            a href=(url_for!(r, "random")) "random Rustacean"
            "."
        }
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

fn search_form(r: &Request, value: &str) -> Markup {
    html! {
        form action=(url_for!(r, "search")) {
            input name="q" id="q" type="search" placeholder="Search"
                autocomplete="off" value=(value) /
        }
        script (PreEscaped(r#"
            var searchBox = document.getElementById('q')
            if (!searchBox.value) searchBox.select()
        "#))
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
        (search_form(r, ""))
    })
}

pub fn search_results<'u, I>(
    r: &Request, query: &str, results: I, correction: Option<String>) -> Markup where
    I: Iterator<Item=(Result<&'u User, &'u str>, String, u64)>,
{
    let title = format!("Search results for “{}”", query);
    let mut results = results.peekable();
    layout_inner(r, Some(&title), None, html! {
        (search_form(r, query))
        @if results.peek().is_none() {
            p "No results found."
        } @else if let Some(correction) = correction {
            p {
                "Showing results for "
                a href=(url_for!(r, "search", "q" => &correction[..])) strong (correction)
            }
        }
        @for (user, id, weight) in results {
            h3 title={ "Weight: " (weight) } {
                a href=(url_for!(r, "user", "id" => &id[..])) {
                    (user_title(&id, user.ok()))
                }
            }
            @if let Ok(user) = user {
                (user_box(&id, user, 3))
            }
            hr /
        }
    })
}

pub fn user(r: &Request, id: &str, user: &User) -> Markup {
    layout(r, Some(&user_title(id, Some(user))), user_box(id, user, 2))
}

fn user_title(id: &str, user: Option<&User>) -> String {
    if let Some(name) = user.and_then(|user| user.name.as_ref()) {
        format!("{} ({})", name, id)
    } else {
        id.to_string()
    }
}

fn user_box(id: &str, user: &User, demote_headers: u32) -> Markup {
    html! {
        table {
            tr {
                th "GitHub"
                td a href={ "https://github.com/" (id) } (id)
            }
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
            div.notes (Markdown { text: x, demote_headers: demote_headers })
        }
    }
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

struct Markdown<'a> {
    text: &'a str,
    demote_headers: u32,
}

impl<'a> Render for Markdown<'a> {
    fn render(&self) -> Markup {
        let parser = Parser::new_ext(self.text, pulldown_cmark::OPTION_ENABLE_TABLES);
        // Demote headers
        let parser = parser.map(|event| match event {
            Event::Start(Tag::Header(level)) =>
                Event::Start(Tag::Header(level + self.demote_headers as i32)),
            Event::End(Tag::Header(level)) =>
                Event::End(Tag::Header(level + self.demote_headers as i32)),
            _ => event,
        });
        let mut unsafe_html = String::new();
        pulldown_cmark::html::push_html(&mut unsafe_html, parser);
        let safe_html = ammonia::clean(&unsafe_html);
        PreEscaped(safe_html)
    }
}
