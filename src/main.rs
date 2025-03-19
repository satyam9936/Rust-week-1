use chrono::{Date, DateTime, Local, Utc};

fn main() {
    let utc: DateTime<Utc>=Utc::now();
    let local_time:DateTime<Local>=Local::now();
print!("{}",utc)
}
