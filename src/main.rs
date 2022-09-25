use anyhow::{anyhow, Result};
use colored::Colorize;
use indicatif::ProgressBar;
use offline::get_offline_followers;
use online::get_online_followers;
use std::{env::args, time::Duration};

use crate::offline::save_offline_followers;

mod offline;
mod online;

fn main() -> Result<()> {
    let username = args().nth(1).ok_or(anyhow!("No Username Provided"))?;

    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(100));
    pb.set_message(format!("Fetching followers for {}...", &username));

    let online_followers = get_online_followers(&username)?;

    pb.finish_and_clear();

    if let Some(offline_followers) = get_offline_followers() {
        let traitors: Vec<_> = offline_followers.difference(&online_followers).collect();
        if traitors.len() == 0 {
            println!("{}", "No one unfollowed you!".green())
        } else {
            println!("{}", "These traitors unfollowed you:".red());
            for traitor in traitors {
                println!(
                    " - {} {}",
                    traitor,
                    format!("(https://github.com/{})", traitor).dimmed()
                );
            }
            println!();
        }
    }

    save_offline_followers(online_followers)?;
    println!("{}", "Saved new follower list.".dimmed());
    Ok(())
}
