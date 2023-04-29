use std::time::{Duration, SystemTime};

use serde::{Deserialize, Serialize};
use serde_json::Value;

pub(super) mod response {
    use serde_json::Value;

    #[derive(Debug)]
    /// Custom response to provide just useful data.
    pub struct MyustResponse {
        /// The JSON output, if any.
        pub json: Option<Value>,
        /// The status code.
        pub status_code: u16,
    }
}

/// The paste's expiration time.
///
/// Examples:
///
/// - 1 day and 12 hours:
///
/// `Expiry { days: 1, hours: 12, ..default::Default() }`
///
/// - 1 hour, 20 minutes and 40 seconds:
///
/// `Expiry { hours: 1, minutes: 20, seconds: 40, ..default::Default() }`
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct Expiry {
    /// The expiration days.
    pub days: i32,
    /// The expiration hours.
    pub hours: i32,
    /// The expiration minutes.
    pub minutes: i32,
    /// The expiration seconds.
    pub seconds: i32,
}

impl Expiry {
    fn total(&self) -> Duration {
        let days = self.days * 24 * 60 * 60;
        let hours = self.hours * 60 * 60;
        let minutes = self.minutes * 60;
        Duration::from_secs((days + hours + minutes + self.seconds) as u64)
    }

    fn add(&self) -> SystemTime {
        let current_time = SystemTime::now();
        match current_time.checked_add(self.total()) {
            Some(new_time) => new_time,
            None => current_time, // handle overflow case
        }
    }

    fn to_vec(&self) -> Vec<(&str, i32)> {
        vec![
            ("days", self.days),
            ("hours", self.hours),
            ("minutes", self.minutes),
            ("seconds", self.seconds),
        ]
    }

    pub(crate) fn invalid_field(&self) -> (String, i32) {
        let expiry_vec = self.to_vec();
        for field in &expiry_vec {
            if field.1 < 0 {
                return (field.0.to_string(), field.1);
            }
        }
        (expiry_vec[0].0.to_string(), expiry_vec[0].1)
    }

    pub(crate) fn valid(&self) -> bool {
        self.days >= 0 && self.hours >= 0 && self.minutes >= 0 && self.seconds >= 0
    }

    pub(crate) fn is_default(&self) -> bool {
        *self == Self::default()
    }

    pub(crate) fn to_rfc3339(&self) -> String {
        let form = humantime::format_rfc3339(self.add()).to_string();
        form.replace("00Z", "+00:00")
    }
}

/// An error received from the API.
#[derive(Debug)]
pub struct MystbinError {
    /// The status code.
    pub code: u16,
    /// The error message, if any.
    pub error: Option<String>,
    /// The notice message, if any.
    pub notice: Option<String>,
    /// The detail of the error, if any.
    pub detail: Option<Value>,
}

/// The base file.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct File {
    /// The file's name.
    pub filename: String,
    /// The file's content.
    pub content: String,
}

/// The base paste.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Paste {
    /// The paste's creation date.
    pub created_at: String,
    /// The paste's expiration date, if any.
    pub expires: Option<Expiry>,
    /// The paste's files.
    pub files: Vec<File>,
    /// The paste's ID.
    pub id: String,
}

/// The paste result from the API.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PasteResult {
    /// The paste's creation date.
    pub created_at: String,
    /// The paste's expiration date, if any.
    pub expires: Option<String>,
    /// The paste's files.
    pub files: Vec<File>,
    /// The paste's ID.
    pub id: String,
}

/// The result obtained from delete_paste and delete_pastes functions.
#[derive(Debug, Default)]
pub struct DeleteResult {
    /// The successfully deleted pastes.
    pub succeeded: Option<Vec<String>>,
    /// The failed pastes to delete.
    pub failed: Option<Vec<String>>,
}

/// The base user paste. This does not contain the files from the paste.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserPaste {
    /// The paste's creation date.
    pub created_at: String,
    /// The paste's expiration date, if any.
    pub expires: Option<String>,
    /// The paste's ID.
    pub id: String,
}
