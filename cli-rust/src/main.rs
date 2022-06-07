use serde::{Deserialize, Serialize};
use std::{thread, time};

#[derive(Serialize, Deserialize)]
struct User {
    first_name: String,
    last_name: String,
    text: String,
    year: i64,
    happy: bool,
    skills: Vec<String>,
}

fn main() {
    for i in 0..=10_000_000 {
        let json_str ="{\"first_name\": \"sergii\",\"last_name\": \"onufriienko\",\"year\": 2022,\"skills\": [\"node.js\", \"go\", \"rust\", \"aws\", \"k8s\"],\"happy\": true,\"text\": \"heppy text, heppy text, heppy text, heppy text, heppy text, heppy text, heppy text, heppy text, heppy text, heppy text, heppy text, heppy text, heppy text, heppy text, heppy text, heppy text, heppy text, heppy text, heppy text, heppy text, heppy text, heppy text, heppy text\"}";
        let deserialized: User = serde_json::from_str(&json_str).unwrap();
    }

    println!("Done");
    thread::sleep(time::Duration::from_secs(60));
    println!("End");
}
