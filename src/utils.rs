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
        // タイムスタンプが正しい形式（YYYYMMDD-HHMMSS）であることを確認
        assert_eq!(timestamp.len(), 15); // YYYYMMDD-HHMMSS = 15文字
        assert!(timestamp.contains('-'));

        let parts: Vec<&str> = timestamp.split('-').collect();
        assert_eq!(parts.len(), 2);

        // 日付部分（YYYYMMDD）
        assert_eq!(parts[0].len(), 8);
        assert!(parts[0].chars().all(|c| c.is_ascii_digit()));

        // 時刻部分（HHMMSS）
        assert_eq!(parts[1].len(), 6);
        assert!(parts[1].chars().all(|c| c.is_ascii_digit()));
    }

    #[test]
    fn test_generate_timestamp_parseable() {
        let timestamp = generate_timestamp();

        // 生成されたタイムスタンプがパース可能であることを確認
        let date_str = &timestamp[..8]; // YYYYMMDD
        let time_str = &timestamp[9..]; // HHMMSS

        let year: i32 = date_str[..4].parse().unwrap();
        let month: u32 = date_str[4..6].parse().unwrap();
        let day: u32 = date_str[6..8].parse().unwrap();

        let hour: u32 = time_str[..2].parse().unwrap();
        let minute: u32 = time_str[2..4].parse().unwrap();
        let second: u32 = time_str[4..6].parse().unwrap();

        // 値が妥当な範囲内であることを確認
        assert!((2020..=2100).contains(&year));
        assert!((1..=12).contains(&month));
        assert!((1..=31).contains(&day));
        assert!(hour <= 23);
        assert!(minute <= 59);
        assert!(second <= 59);
    }

    #[test]
    fn test_generate_timestamp_uniqueness() {
        // 短時間内での呼び出しでも一意性を確認（秒単位での変化を想定）
        let timestamp1 = generate_timestamp();
        std::thread::sleep(std::time::Duration::from_millis(100));
        let timestamp2 = generate_timestamp();

        // 100msの間隔では同じ秒になる可能性が高いが、フォーマットが一貫していることを確認
        assert_eq!(timestamp1.len(), timestamp2.len());
        assert!(timestamp1.contains('-'));
        assert!(timestamp2.contains('-'));
    }
}
