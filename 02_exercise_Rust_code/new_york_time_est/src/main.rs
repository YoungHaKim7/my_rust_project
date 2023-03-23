use chrono::{DateTime, FixedOffset, Local, Utc};

fn main() {
    let local_time = Local::now();
    let utc_time = DateTime::<Utc>::from_utc(local_time.naive_utc(), Utc);
    let newyork_timezone = FixedOffset::west(4 * 3600);
    let ny_string: String = newyork_timezone.to_string();
    println!(
        "newyork_timezone: {}",
        utc_time.with_timezone(&newyork_timezone)
    );
}
