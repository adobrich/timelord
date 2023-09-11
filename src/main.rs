use chrono::{Local, NaiveDateTime};

fn main() {
    println!("{:?}", Local::now().naive_local());
}
