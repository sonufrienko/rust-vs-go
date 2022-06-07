use std::{thread, time};

struct User {
    first_name: String,
    last_name: String,
    text: String,
    year: i64,
    happy: bool,
    skills: Vec<String>,
}

fn run() {
    let mut mem: Vec<User> = vec![];
    for i in 0..=20_000_000 {
        mem.push(User {
            first_name: "sergii".to_string(),
            last_name: "onufriienko".to_string(),
            year: i as i64,
            skills: vec![
                "node.js".to_string(),
                "go".to_string(),
                "rust".to_string(),
                "aws".to_string(),
                "k8s".to_string(),
            ],
            happy: true,
            text: "heppy text".to_string(),
        })
    }
}

fn main() {
    run();

    // println!("Done");
    // thread::sleep(time::Duration::from_secs(60));
    // println!("End");
}
