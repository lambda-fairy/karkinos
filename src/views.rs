use std::fmt;

use iron_maud::Template;

pub fn default(title: String, body: Template) -> Template {
    Template::new(move |w| html!(*w, {
        $$"<!DOCTYPE html>"
        html {
            head {
                title $title
                style $$ r#"html { font-family: "Comic Sans MS" }"#
            }
            body {
                h1 $title
                #call body
            }
        }
    }))
}

pub fn home() -> Template {
    Template::new(move |w| html!(*w, {
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
    Template::new(move |w| html!(*w, {
        p {
            "The page at "
            strong $url
            " could not be found."
        }
        p a href="/" "<< Back to home page"
    }))
}

pub fn user(user: User) -> Template {
    let make_row = |key: &'static str, value: Option<String>| move |w: &mut fmt::Write|
        html!(*w, #if let Some(ref value) = value {
            tr {
                th $key
                td $value
            }
        });
    Template::new(move |w| html!(*w, {
        table {
            #call make_row("Name", user.name)
            #call make_row("IRC nick", user.irc)
            #if !user.irc_channels.is_empty() {
                tr {
                    th "IRC channels"
                    td #for (i, channel) in user.irc_channels.iter().enumerate() {
                        #if i > 0 { ", " }
                        $channel
                    }
                }
            }
            #call make_row("Email", user.email)
            #call make_row("Discourse", user.discourse)
            #call make_row("Notes", user.notes)
        }
    }))
}

pub fn user_not_found(name: String) -> Template {
    Template::new(move |w| html!(*w, {
        p {
            "The user "
            strong $name
            " could not be found."
        }
        p a href="/" "<< Back to home page"
    }))
}
