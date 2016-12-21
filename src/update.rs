use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::thread::{self, JoinHandle};
use std::time::Duration;

const REPO_URL: &'static str = "https://github.com/nrc/rustaceans.org.git";

pub struct Updater {
    path: PathBuf,
    worker: JoinHandle<()>,
}

impl Updater {
    pub fn start<P: AsRef<Path>>(root: P) -> io::Result<Updater> {
        let Output { status, stdout, .. } = git().arg("--version").output()?;
        if !status.success() {
            return Err(io::Error::new(io::ErrorKind::Other, "failed to check git version"));
        }
        info!("found git: {}", String::from_utf8_lossy(&stdout).trim());

        let path = root.as_ref().join("data");
        if !path.is_dir() {
            // Clone the repo
            info!("cloning rustaceans data");
            // TODO: only forward output on error
            let status = git().arg("clone").arg(REPO_URL).arg(&path).status()?;
            if !status.success() {
                return Err(io::Error::new(io::ErrorKind::Other, "failed to clone data repository"));
            }
        }

        let worker = {
            let path = path.clone();
            // TODO: stop flag
            thread::spawn(move || loop {
                thread::sleep(Duration::from_secs(5 * 60));
                info!("updating rustaceans data");
                match git().arg("pull").arg("--ff-only").current_dir(&path).status() {
                    Ok(status) if status.success() => {},
                    Ok(status) => error!("update failed with exit status {}", status),
                    Err(e) => error!("update failed with error: {}", e),
                }
                info!("updated successfully");
            })
        };

        Ok(Updater {
            path: path,
            worker: worker,
        })
    }

    pub fn data_dir(&self) -> &Path {
        &self.path
    }
}

fn git() -> Command {
    Command::new("git")
}
