use chrono::{DateTime, FixedOffset};
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
    pub created_at: DateTime<FixedOffset>,
    /// The paste's expiration date, if any.
    pub expires: Option<DateTime<FixedOffset>>,
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
    pub created_at: DateTime<FixedOffset>,
    /// The paste's expiration date, if any.
    pub expires: Option<DateTime<FixedOffset>>,
    /// The paste's ID.
    pub id: String,
}
