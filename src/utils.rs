pub(super) mod utils {
    use chrono::{DateTime, FixedOffset};
    
    pub const ENDPOINT_URL: &str = "https://api.mystb.in";

    pub fn parse_date(date: &str) -> DateTime<FixedOffset> {
        DateTime::parse_from_rfc3339(date).unwrap()
    }
}
