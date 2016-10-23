use serde_json::{self, Value};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Debug)]
pub struct User {
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
            let mut path = PathBuf::from("data");
            path.push(name);
            path.set_extension("json");
            path
        };
        let reader = BufReader::new(File::open(path)?);
        let user = User::from_json(serde_json::from_reader(reader)?);
        Ok(user)
    }

    #[allow(dead_code)]
    pub fn from_str(json: &str) -> Result<User, serde_json::Error> {
        serde_json::from_str(json).map(User::from_json)
    }

    pub fn from_json(json: Value) -> User {
        User {
            name: json.find("name").and_then(Value::as_str).map(String::from),
            irc: json.find("irc").and_then(Value::as_str).map(String::from),
            irc_channels: json.find("irc_channels").and_then(Value::as_array)
                .and_then(|chans| chans.iter()
                    .map(|json| json.as_str().map(String::from))
                    .collect::<Option<Vec<String>>>())
                .unwrap_or(Vec::new()),
            show_avatar: json.find("show_avatar").and_then(Value::as_bool).unwrap_or(false),
            email: json.find("email").and_then(Value::as_str).map(String::from),
            discourse: json.find("discourse").and_then(Value::as_str).map(String::from),
            reddit: json.find("reddit").and_then(Value::as_str).map(String::from),
            twitter: json.find("twitter").and_then(Value::as_str).map(String::from),
            blog: json.find("blog").and_then(Value::as_str).map(String::from),
            website: json.find("website").and_then(Value::as_str).map(String::from),
            notes: json.find("notes").and_then(Value::as_str).map(String::from),
        }
    }
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
    User::from_str(DATA).unwrap();
}
