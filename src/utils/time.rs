use chrono::{DateTime, NaiveDateTime, Utc};

/// 获取北京时间（UTC+8）
pub fn now() -> NaiveDateTime {
    let utc_now: DateTime<Utc> = Utc::now();
    let shanghai = chrono_tz::Asia::Shanghai;
    utc_now.with_timezone(&shanghai).naive_local()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_now_is_beijing_time() {
        let time = now();
        println!("Beijing time: {}", time);
    }
}
