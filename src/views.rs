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
