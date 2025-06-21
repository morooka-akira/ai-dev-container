use chrono::{DateTime, Local};

#[allow(dead_code)]
pub fn generate_timestamp() -> String {
    let now: DateTime<Local> = Local::now();
    now.format("%Y%m%d-%H%M%S").to_string()
}
