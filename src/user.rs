use serde_json;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
pub struct User {
    // NOTE: when changing these fields, be sure to update
    // `.remove_empty_strings()` below
    pub name: Option<String>,
    pub irc: Option<String>,
    pub irc_channels: Vec<String>,
    pub show_avatar: bool,
    pub email: Option<String>,
    pub discourse: Option<String>,
    pub reddit: Option<String>,
    pub twitter: Option<String>,
    pub blog: Option<String>,
    pub website: Option<String>,
    pub notes: Option<String>,
}

impl User {
    pub fn lookup(name: &str) -> Result<User, serde_json::Error> {
        let path = {
            let mut path = PathBuf::from("rustaceans.org/data");
            path.push(name);
            path.set_extension("json");
            path
        };
        let reader = BufReader::new(File::open(path)?);
        let mut user: User = serde_json::from_reader(reader)?;
        user.remove_empty_strings();
        Ok(user)
    }

    /// Some users use empty strings to mean "not applicable", instead of using
    /// `null` or omitting the field as they should be doing. We fix up their
    /// carelessness here.
    fn remove_empty_strings(&mut self) {
        macro_rules! fixup {
            ($($field:ident)*) => {
                $(
                    // If the field is just whitespace, replace it with `None`
                    // This boolean dance is needed to satisfy borrowck
                    let should_replace = match self.$field {
                        Some(ref s) if is_whitespace(s) => true,
                        _ => false,
                    };
                    if should_replace {
                        self.$field = None;
                    }
                )*
            }
        }
        fixup!(name irc email discourse reddit twitter blog website notes);
    }
}

fn is_whitespace(s: &str) -> bool {
    s.chars().all(char::is_whitespace)
}

#[test]
fn smoke() {
    const DATA: &'static str = r#"
        {
            "name": "Bors",
            "show_avatar": true,
            "irc": "bors",
            "irc_channels": ["rust", "rust-bots"],
            "website": "http://rust-lang.org/"
        }
    "#;
    serde_json::from_str(DATA).unwrap();
}
