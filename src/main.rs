use std::collections::HashMap;
use std::sync::Arc;
use std::fs;
use std::time::Instant;

use serde::Deserialize;
use serde::de::{self, Deserializer};
use chrono::DateTime;
use http::HeaderMap;
use tokio::time::{Duration, sleep};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
enum Method {
    Get,
    Post,
    Patch,
    Put
}

#[derive(Deserialize, Debug)]
struct Url {
    path: String
}

#[derive(Deserialize, Debug)]
struct Request {
    headers: HashMap<String, String>,
    method: Method
}

#[derive(Deserialize, Debug)]
struct Http {
    request: Request
}

#[derive(Deserialize, Debug)]
struct Record {
    #[serde(rename = "@timestamp", deserialize_with = "from_timestamp_to_unix")]
    timestamp: u64,
    http: Http,
    url: Url,
}

fn from_timestamp_to_unix<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let timestamp_str = String::deserialize(deserializer)?;
    match DateTime::parse_from_rfc3339(&timestamp_str) {
        Ok(dt) => Ok(dt.timestamp_millis() as u64),
        Err(_) => Err(de::Error::custom("Invalid timestamp format"))
    }
}

async fn send_request(client: Arc<reqwest::Client>, record: Record, delta: u64) -> Result<reqwest::Response, reqwest::Error> {
    let start = Instant::now();
    sleep(Duration::from_millis(delta)).await;
    let end = Instant::now();
    let jitter = (end - start) - Duration::from_millis(delta);

    println!(
        "Jitter for {}: {:?} ({:?}, {:?}, {:?}ms, {:?}ms)",
        record.url.path,
        jitter,
        end,
        start,
        delta,
        end.duration_since(start).as_millis()
    );

    println!(
        "Sending request to {:?} {}",
        record.http.request.method,
        record.url.path
    );

    let headers: HeaderMap = (&record.http.request.headers).try_into().expect("valid headers");
    let url = format!("http://bin-web-app:8080/{}", record.url.path);
    let builder = match record.http.request.method {
        Method::Get => client.get(url),
        Method::Post => client.post(url),
        Method::Patch => client.patch(url),
        Method::Put => client.put(url)
    };

    builder.headers(headers).send().await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = fs::read_to_string("./test-data.json")?;

    let mut de: Vec<Record> = serde_json::from_str(&data)?;

    de.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

    let start_timestamp = de[0].timestamp;
    let client = Arc::new(reqwest::Client::new());

    let mut tasks = Vec::new();

    for record in de {
        let client_clone = Arc::clone(&client);
        let delta = record.timestamp - start_timestamp;

        let task = tokio::spawn(async move {
            if let Err(err) = send_request(client_clone, record, delta).await {
                eprintln!("Error sending request: {:?}", err);
            }
        });

        tasks.push(task);
    };

    for task in tasks {
        task.await?;
    }

    Ok(())
}
