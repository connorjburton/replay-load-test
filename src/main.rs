use std::time::Duration;
use std::thread;
use std::collections::HashMap;

use serde::Deserialize;
use serde::de::{self, Deserializer};
use chrono::DateTime;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
enum Method {
    Get,
    Post,
    Patch
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

fn main() -> Result<(), serde_json::Error> {
    let data = r#"[
    {
        "@timestamp": "2015-01-01T08:00:07.753Z",
        "http": {
            "request": {
                "headers": {
                    "User-Agent": "Mozilla/5.0 (X11; OpenBSD amd64; rv:28.0) Gecko/20100101 Firefox/28.0"
                },
                "method": "POST"
            }
        },
        "url": {
            "path": "bad.png"
        }
    },
    {
        "@timestamp": "2015-01-01T08:03:30.249Z",
        "http": {
            "request": {
                "headers": {
                    "User-Agent": "Mozilla/5.0 (Windows NT 6.1; WOW64; Trident/7.0; AS; rv:11.0) like Gecko"
                },
                "method": "GET"
            }
        },
        "url": {
            "path": "large.mp3"
        }
    },
    {
        "@timestamp": "2015-01-01T08:00:11.635Z",
        "http": {
            "request": {
                "headers": {
                    "User-Agent": "Mozilla/5.0 (Windows NT 6.1; WOW64; rv:40.0) Gecko/20100101 Firefox/40.1"
                },
                "method": "PATCH"
            }
        },
        "url": {
            "path": "olivia.jpg"
        }
    }
    ]"#;

    let mut de: Vec<Record> = serde_json::from_str(&data)?;

    de.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

    let start_timestamp = de[0].timestamp;

    let handles: Vec<_> = de.into_iter().map(|record| {
        thread::spawn(move || {
            println!("sleeping for {}ms", record.timestamp - start_timestamp);
            thread::sleep(Duration::from_millis(record.timestamp - start_timestamp));
            println!(
                "going to send request for {}",
                record.url.path
            );
        })
    }).collect();

    for handle in handles {
        handle.join().unwrap();
    }

    // for record in de.iter() {
    //     let record_clone = record.clone();
    //     println!("{:#?}", record);
    //         // record,
    //         // delta: record.timestamp - start_timestamp

    //     let handle = thread::spawn(move || {
    //         println!("going to send request, delta {}", record_clone.timestamp - start_timestamp);
    //     });

    //     handle.join().unwrap();
    // }

    Ok(())
}
