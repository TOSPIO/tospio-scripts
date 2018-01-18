extern crate regex;

use std::cmp::Ordering;
use std::fs::DirEntry;
use std::io::{ErrorKind, Write, stdout};
use std::path::Path;
use regex::Regex;

#[derive(Eq, Ord)]
struct PSFD {
    pid: u32,
    cnt: usize
}

impl PartialEq for PSFD {
    fn eq(&self, other: &Self) -> bool {
        return self.cnt == other.cnt;
    }
}

impl PartialOrd for PSFD {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cnt.cmp(&other.cnt));
    }
}

impl PSFD {
    fn format(&self) -> String {
        return format!("{}\t{}", self.pid, self.cnt);
    }
}

fn psfd(entry: &DirEntry) -> Option<PSFD> {
    let path = entry.path();
    let fd_path = path.join(Path::new("fd"));
    let pid = path.file_name().unwrap().to_str().unwrap();
    if let Ok(read_dir) = fd_path.read_dir() {
        return Some(
            PSFD {
                pid: pid.parse().unwrap(),
                cnt: read_dir.count()
            }
        );
    }
    return None;
}

fn psfds() -> Vec<PSFD> {
    let pid_regex = Regex::new(r"^\d+$").unwrap();
    let proc_ = Path::new("/proc");
    let mut psfds = Vec::new();
    for entry in proc_.read_dir().expect("read_dir call failed") {
        if let Ok(entry) = entry {
            let file_name = entry.file_name().into_string().unwrap();
            if pid_regex.is_match(file_name.as_str()) {
                if let Some(psfd) = psfd(&entry) {
                    psfds.push(psfd);
                }
            }
        }
    }
    return psfds;
}

fn main() {
    let mut psfds = psfds();
    psfds.sort();
    let mut stdout = stdout();
    for psfd in psfds.iter().rev() {
        if let Err(err) = writeln!(stdout, "{}", psfd.format()) {
            if err.kind() == ErrorKind::BrokenPipe {
                return;
            }
            panic!(err);
        }
    }
}
