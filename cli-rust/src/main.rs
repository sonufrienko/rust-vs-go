use std::{thread, time};

fn main() {
    println!("Start");
    thread::sleep(time::Duration::from_secs(60));
    println!("End");
}
