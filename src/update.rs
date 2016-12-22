use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::{self, JoinHandle};
use std::time::Duration;

const REPO_URL: &'static str = "https://github.com/nrc/rustaceans.org.git";

pub struct Updater {
    path: PathBuf,
    keep_going: Arc<AtomicBool>,
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
            let status = git().arg("clone").arg(REPO_URL).arg(&path).status()?;
            if !status.success() {
                return Err(io::Error::new(io::ErrorKind::Other, "failed to clone data repository"));
            }
        }

        let keep_going = Arc::new(AtomicBool::new(true));

        let worker = {
            let path = path.clone();
            let keep_going = keep_going.clone();
            thread::spawn(move || while keep_going.load(Ordering::SeqCst) {
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
            keep_going: keep_going,
            worker: worker,
        })
    }

    pub fn data_dir(&self) -> &Path {
        &self.path
    }

    #[allow(dead_code)]  // FIXME(#15)
    pub fn stop(self) -> thread::Result<()> {
        self.keep_going.store(false, Ordering::SeqCst);
        self.worker.join()
    }
}

fn git() -> Command {
    Command::new("git")
}
