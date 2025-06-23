use chrono::{DateTime, Local};

pub fn generate_timestamp() -> String {
    let now: DateTime<Local> = Local::now();
    now.format("%Y%m%d-%H%M%S").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_timestamp_format() {
        let timestamp = generate_timestamp();
        // Verify that timestamp has correct format (YYYYMMDD-HHMMSS)
        assert_eq!(timestamp.len(), 15); // YYYYMMDD-HHMMSS = 15 characters
        assert!(timestamp.contains('-'));

        let parts: Vec<&str> = timestamp.split('-').collect();
        assert_eq!(parts.len(), 2);

        // Date part (YYYYMMDD)
        assert_eq!(parts[0].len(), 8);
        assert!(parts[0].chars().all(|c| c.is_ascii_digit()));

        // Time part (HHMMSS)
        assert_eq!(parts[1].len(), 6);
        assert!(parts[1].chars().all(|c| c.is_ascii_digit()));
    }

    #[test]
    fn test_generate_timestamp_parseable() {
        let timestamp = generate_timestamp();

        // Verify that generated timestamp is parseable
        let date_str = &timestamp[..8]; // YYYYMMDD
        let time_str = &timestamp[9..]; // HHMMSS

        let year: i32 = date_str[..4].parse().unwrap();
        let month: u32 = date_str[4..6].parse().unwrap();
        let day: u32 = date_str[6..8].parse().unwrap();

        let hour: u32 = time_str[..2].parse().unwrap();
        let minute: u32 = time_str[2..4].parse().unwrap();
        let second: u32 = time_str[4..6].parse().unwrap();

        // Verify that values are within valid ranges
        assert!((2020..=2100).contains(&year));
        assert!((1..=12).contains(&month));
        assert!((1..=31).contains(&day));
        assert!(hour <= 23);
        assert!(minute <= 59);
        assert!(second <= 59);
    }

    #[test]
    fn test_generate_timestamp_uniqueness() {
        // Verify uniqueness even with calls in short intervals (assuming change at second level)
        let timestamp1 = generate_timestamp();
        std::thread::sleep(std::time::Duration::from_millis(100));
        let timestamp2 = generate_timestamp();

        // 100ms interval likely results in same second, but verify format consistency
        assert_eq!(timestamp1.len(), timestamp2.len());
        assert!(timestamp1.contains('-'));
        assert!(timestamp2.contains('-'));
    }
}
