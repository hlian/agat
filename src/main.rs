extern crate serde;

use chrono::{DateTime, Utc};
use failure::{bail, err_msg, Error};
use github_rs::client::{Executor, Github};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
struct User {
    login: String
}

#[derive(Serialize, Deserialize, Debug)]
struct PR {
    title: String,
    user: User,
    created_at: DateTime<Utc>,
}

fn main2(token: &str) -> Result<(), Error> {
    let client = Github::new(token).map_err(failure::SyncFailure::new)?;
    let (_headers, status, json) = client
        .get()
        .repos()
        .owner("Originate")
        .repo("dashboard")
        .pulls()
        .execute::<Vec<PR>>()
        .map_err(failure::SyncFailure::new)?;
    if status != 200 {
        bail!("nope {}", status);
    }
    println!("{:?}", &json.ok_or_else(|| err_msg("bummer"))?);
    Ok(())
}

fn main() -> Result<(), Error> {
    match std::env::var_os("TOKEN").and_then(|s| s.into_string().ok()) {
        Some(token2) => main2(&token2),
        None => {
            eprintln!("Missing TOKEN");
            bail!("Missing TOKEN")
        }
    }
}
