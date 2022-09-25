use anyhow::{bail, Context, Result};
use reqwest::blocking::ClientBuilder;
use serde::Deserialize;
use std::collections::HashSet;

// I know this doesn't have every field, but Rust complains if I leave them in
#[derive(Deserialize)]
struct APIFollower {
    login: String,
}

pub fn get_online_followers(username: &str) -> Result<HashSet<String>> {
    let mut followers = HashSet::new();
    let mut page = 1;
    let client = ClientBuilder::new().user_agent("traitor").build()?;

    loop {
        let res = client
            .get(format!(
                "https://api.github.com/users/{}/followers",
                username
            ))
            .query(&[("per_page", "100"), ("page", &page.to_string())])
            .send()
            .context("Request failed - are you connected to the internet?")?;

        if !res.status().is_success() {
            bail!(
                "Unsuccessful response with status code {} - is the username correct?",
                res.status().as_u16()
            );
        }

        let json: Vec<APIFollower> = res.json()?;

        if json.len() == 0 {
            break;
        };

        for follower in json {
            followers.insert(follower.login);
        }
        page += 1;
    }

    Ok(followers)
}
