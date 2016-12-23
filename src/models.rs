use rand::{self, Rng};
use serde_json;
use std::collections::BTreeMap;
use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::BufReader;
use std::path::Path;

use search::SearchIndex;

#[derive(Deserialize, Debug)]
pub struct User {
    // NOTE: when changing these fields, be sure to update
    // `.remove_empty_strings()` and `.with_str_fields()` below
    pub name: Option<String>,
    pub irc: Option<String>,
    #[serde(default)]
    pub irc_channels: Vec<String>,
    #[serde(default)]
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
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<User, serde_json::Error> {
        let reader = BufReader::new(File::open(path)?);
        let mut user: User = serde_json::from_reader(reader)?;
        user.remove_empty_strings();
        for channel in &mut user.irc_channels {
            if channel.starts_with('#') {
                *channel = channel.trim_left_matches('#').to_string();
            }
        }
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

    /// Applies the given callback to every searchable field in this entry.
    ///
    /// Used by the full-text search machinery.
    fn with_str_fields<F>(&self, mut callback: F) where F: FnMut(&str, u64) {
        macro_rules! callme {
            ($callback:ident, $($field:ident ($weight:expr))*) => {
                $(
                    if let Some(ref s) = self.$field {
                        $callback(s, $weight);
                    }
                )*
            }
        }
        callme!(
            callback,
            // Give names more weight than URLs and notes
            name(20) irc(10) email(1) discourse(10) reddit(10)
            twitter(10) blog(1) website(1) notes(1)
            );
        for channel in &self.irc_channels {
            callback(channel, 1);
        }
    }
}

fn is_whitespace(s: &str) -> bool {
    s.chars().all(char::is_whitespace)
}

#[derive(Debug)]
pub struct Users {
    data: BTreeMap<String, Result<User, String>>,
    index: SearchIndex<String>,
}

impl Users {
    pub fn load<P: AsRef<Path>>(data_dir: P) -> Result<Users, serde_json::Error> {
        let mut data = BTreeMap::new();
        for entry in fs::read_dir(data_dir.as_ref())? {
            let path = entry?.path();
            if path.extension() == Some(OsStr::new("json")) {
                let id = path.file_stem().unwrap().to_string_lossy().into_owned();
                // Some users' entries actually fail to parse!
                // Instead of bailing on these, just record the error and move on.
                let user = User::from_path(&path).map_err(|e| {
                    warn!("could not parse entry for {}: {}", id, e);
                    e.to_string()
                });
                data.insert(id, user);
            }
        }
        let mut index = SearchIndex::new();
        for (id, user) in &data {
            index.add(id.clone(), id, 10);
            if let Ok(ref user) = *user {
                user.with_str_fields(|s, w| index.add(id.clone(), s, w));
            }
        }
        info!("loaded {} rustaceans", data.len());
        Ok(Users { data: data, index: index })
    }

    pub fn random_id(&self) -> Option<&str> {
        let i = rand::thread_rng().gen_range(0, self.data.len());
        self.data.keys().nth(i).map(AsRef::as_ref)
    }

    pub fn get(&self, id: &str) -> Option<Result<&User, &str>> {
        self.data.get(id).map(|r| r.as_ref().map_err(|e| &e[..]))
    }

    pub fn search(&self, query: &str) -> (Vec<(String, u64)>, Option<String>) {
        self.index.query(query)
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
    let _: User = serde_json::from_str(DATA).unwrap();
}
