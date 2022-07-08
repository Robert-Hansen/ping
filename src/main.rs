use futures::{stream, StreamExt};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::time::{Duration, Instant};
use tabled::{Footer, Header, Style, Table, Tabled};

#[derive(Debug, Tabled, Serialize, Deserialize, Clone)]
struct Domain {
    url: String,
    #[tabled(display_with = "display_option")]
    time: Option<u128>,
}

fn display_option(o: &Option<u128>) -> String {
    match o {
        Some(s) => format!("{}", s),
        None => format!(""),
    }
}

async fn compute_job(domain: Domain) -> Domain {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap();

    let time = Instant::now();

    let body = client.head(domain.url.to_string()).send().await;

    let time_elapsed = time.elapsed().as_millis();

    match body {
        Ok(response) => {
            match response.status() {
                reqwest::StatusCode::OK => {}
                _ => {
                    println!(
                        "url -> {} status code {}",
                        domain.url.to_string(),
                        response.status()
                    );
                }
            };
        }
        _ => println!("Default"),
    }

    Domain {
        url: domain.url.to_string(),
        time: Some(time_elapsed),
    }
}

#[tokio::main]
async fn main() {
    let path = Path::new("/usr/src/app/src/domains.json");
    let file = fs::File::open(path).expect("file should open read only");
    let domains: Vec<Domain> = serde_json::from_reader(file).unwrap();
    let concurrency = 10;

    let mut results: Vec<Domain> = stream::iter(domains)
        .map(compute_job)
        .buffer_unordered(concurrency)
        .collect()
        .await;

    results.sort_by_key(|d| d.time);

    let table = Table::new(&results)
        .with(Header("Domains"))
        .with(Footer(format!("{} elements", results.len())))
        .with(Style::modern());

    println!("{}", table);
}
