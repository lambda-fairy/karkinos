use serde_json::{self, Value};

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
    pub fn from_str(json: &str) -> Result<User, serde_json::Error> {
        serde_json::from_str(json).map(User::from_json)
    }

    pub fn from_json(json: Value) -> User {
        User {
            name: json.find("name").and_then(Value::as_string).map(String::from),
            irc: json.find("irc").and_then(Value::as_string).map(String::from),
            irc_channels: json.find("irc_channels").and_then(Value::as_array)
                .and_then(|chans| chans.iter()
                    .map(|json| json.as_string().map(String::from))
                    .collect::<Option<Vec<String>>>())
                .unwrap_or(Vec::new()),
            show_avatar: json.find("show_avatar").and_then(Value::as_boolean).unwrap_or(false),
            email: json.find("email").and_then(Value::as_string).map(String::from),
            discourse: json.find("discourse").and_then(Value::as_string).map(String::from),
            reddit: json.find("reddit").and_then(Value::as_string).map(String::from),
            twitter: json.find("twitter").and_then(Value::as_string).map(String::from),
            blog: json.find("blog").and_then(Value::as_string).map(String::from),
            website: json.find("website").and_then(Value::as_string).map(String::from),
            notes: json.find("notes").and_then(Value::as_string).map(String::from),
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
