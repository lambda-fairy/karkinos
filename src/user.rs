use serde_json;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
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
            let mut path = PathBuf::from("rustaceans.org/data");
            path.push(name);
            path.set_extension("json");
            path
        };
        let reader = BufReader::new(File::open(path)?);
        let user = serde_json::from_reader(reader)?;
        Ok(user)
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
    serde_json::from_str(DATA).unwrap();
}
