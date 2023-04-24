pub(super) mod utils {
    use chrono::{DateTime, FixedOffset};

    pub const BOOKMARK_ENDPOINT: &str = "https://api.mystb.in/users/bookmarks";
    pub const PASTE_ENDPOINT: &str = "https://api.mystb.in/paste";
    pub const SELF_ENDPOINT: &str = "https://api.mystb.in/users/@me";
    pub const USER_PASTES_ENDPOINT: &str = "https://api.mystb.in/pastes/@me";

    pub fn parse_date(date: &str) -> DateTime<FixedOffset> {
        DateTime::parse_from_rfc3339(date).unwrap()
    }
}
