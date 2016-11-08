use iron::prelude::*;
use maud::{Markup, PreEscaped, Render};
use pulldown_cmark;

use models::User;

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
            link rel="stylesheet" href=(url_for!(r, "static", "path" => "styles.css")) /
            body {
                h1 a href="/" {
                    span.thecrab "🦀"
                    "Karkinos"
                }
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
        (search_form(r, None))
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

fn search_form(r: &Request, value: Option<&str>) -> Markup {
    html! {
        form action=(url_for!(r, "search")) {
            input name="q" id="q" type="search" placeholder="Search" value=(value.unwrap_or("")) /
        }
        @if value.is_none() {
            script (PreEscaped("document.getElementById('q').select()"))
        }
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
        (search_form(r, None))
    })
}

pub fn search_results<'u, I>(r: &Request, query: &str, results: I) -> Markup where
    I: Iterator<Item=(Result<&'u User, &'u str>, String, u64)>,
{
    let title = format!("Search results for “{}”", query);
    let results = results.map(|(user, id, weight)| match user {
        Ok(user) => (user_box(r, &id, &user), id, weight),
        Err(..) => ((id.clone(), html! {}), id, weight),
    });
    let mut results = results.peekable();
    layout(r, Some(&title), html! {
        (search_form(r, Some(query)))
        @if results.peek().is_none() {
            p "No results found."
        }
        @for ((user_title, user_markup), id, weight) in results {
            h3 title={ "Weight: " (weight) } {
                a href=(url_for!(r, "user", "id" => &id[..])) (user_title)
            }
            (user_markup)
            hr /
        }
    })
}

pub fn user(r: &Request, id: &str, user: &User) -> Markup {
    let (title, body) = user_box(r, id, user);
    layout(r, Some(&title), body)
}

fn user_box(_r: &Request, id: &str, user: &User) -> (String, Markup) {
    let title = if let Some(ref name) = user.name {
        format!("{} ({})", name, id)
    } else {
        id.to_string()
    };
    (title, html! {
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
