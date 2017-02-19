use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::thread;
use std::time::Duration;

const REPO_URL: &'static str = "https://github.com/nrc/rustaceans.org.git";

pub struct Updater {
    data_dir: PathBuf,
}

impl Updater {
    pub fn start<P: AsRef<Path>>(root: P) -> io::Result<Updater> {
        let Output { status, stdout, .. } = git().arg("--version").output()?;
        if !status.success() {
            return Err(io::Error::new(io::ErrorKind::Other, "failed to check git version"));
        }
        info!("found git: {}", String::from_utf8_lossy(&stdout).trim());

        let repo_dir = root.as_ref().join("data");
        if !repo_dir.is_dir() {
            // Clone the repo
            info!("cloning rustaceans data");
            let status = git().arg("clone").arg(REPO_URL).arg(&repo_dir).status()?;
            if !status.success() {
                return Err(io::Error::new(io::ErrorKind::Other, "failed to clone data repository"));
            }
        }

        {
            let repo_dir = repo_dir.clone();
            thread::spawn(move || loop {
                thread::sleep(Duration::from_secs(5 * 60));
                info!("updating rustaceans data");
                match git().arg("pull").arg("--ff-only").current_dir(&repo_dir).status() {
                    Ok(status) if status.success() => {},
                    Ok(status) => error!("update failed with exit status {}", status),
                    Err(e) => error!("update failed with error: {}", e),
                }
                info!("updated successfully");
            });
        }

        Ok(Updater { data_dir: repo_dir.join("data") })
    }

    pub fn data_dir(&self) -> &Path {
        &self.data_dir
    }
}

fn git() -> Command {
    Command::new("git")
}
