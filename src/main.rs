extern crate serde;

use chrono::{DateTime, Utc};
use failure::{bail, err_msg, Error};
use github_rs::client::{Executor, Github};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    login: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct PR {
    title: String,
    user: User,
    created_at: DateTime<Utc>,
}

#[derive(Debug)]
struct Cache<T> {
    value: T,
    etag: http::header::HeaderValue,
}

fn prs(token: &str, cache: Option<Cache<Vec<PR>>>) -> Result<Cache<Vec<PR>>, Error> {
    let client = Github::new(token).map_err(failure::SyncFailure::new)?;
    let builder = client.get();
    let builder2 = match &cache {
        Some(Cache { etag, .. }) => builder.set_etag(etag),
        None => builder,
    };

    let (headers, status, json) = builder2
        .repos()
        .owner("Originate")
        .repo("dashboard")
        .pulls()
        .execute::<Vec<PR>>()
        .map_err(failure::SyncFailure::new)?;
    if status == 304 {
        Ok(cache.ok_or_else(|| err_msg("impossible"))?)
    } else if status != 200 {
        bail!("nope {}", status);
    } else {
        println!("{:?}", headers);
        Ok(Cache {
            value: json.ok_or_else(|| err_msg("invalid response"))?,
            etag: github_rs::headers::etag(&headers)
                .ok_or_else(|| err_msg("missing etag"))?
                .clone(),
        })
    }
}

fn main2(token: &str) -> Result<(), Error> {
    let cache = prs(token, None)?;
    let etag2 = prs(token, Some(cache))?;
    println!("success {:?}, {}", etag2.value, etag2.etag.to_str()?);
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
