use dirs::home_dir;
use std::{
    collections::HashSet,
    fs::{read_to_string, write},
    io,
    path::PathBuf,
};

fn get_offline_path() -> PathBuf {
    home_dir().unwrap().join(".traitor")
}

pub fn save_offline_followers(followers: HashSet<String>) -> io::Result<()> {
    write(
        get_offline_path(),
        followers
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
            .join("\n"),
    )
}

pub fn get_offline_followers() -> Option<HashSet<String>> {
    if let Some(contents) = read_to_string(get_offline_path()).ok() {
        Some(
            contents
                .split("\n")
                .map(|follower| follower.to_string())
                .collect(),
        )
    } else {
        None
    }
}
