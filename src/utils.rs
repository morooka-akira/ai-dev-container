use chrono::{DateTime, Local};

pub fn generate_timestamp() -> String {
    let now: DateTime<Local> = Local::now();
    now.format("%Y%m%d-%H%M%S").to_string()
}
