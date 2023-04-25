use std::process::Termination;

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

pub(super) mod response {
    use serde_json::Value;

    #[derive(Debug)]
    pub struct MyustResponse {
        pub json: Option<Value>,
        pub status_code: u16,
    }
}

/// An error received from the API.
#[derive(Debug)]
pub struct MystbinError {
    pub code: u16,
}

/// The base file.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct File {
    /// The file's name.
    pub filename: String,
    /// The file's content.
    pub content: String,
}

/// The base paste.
#[derive(Clone, Debug, Default)]
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

impl Termination for Paste {
    fn report(self) -> std::process::ExitCode {
        todo!()
    }
}

/// The result obtained from delete_paste and delete_paste functions.
#[derive(Debug, Default)]
pub struct DeleteResult {
    /// The successfully deleted pastes.
    pub succeeded: Option<Vec<String>>,
    /// The failed pastes to delete.
    pub failed: Option<Vec<String>>,
}

/// The base user paste. This does not contain the files from the paste.
#[derive(Clone, Debug)]
pub struct UserPaste {
    /// The paste's creation date.
    pub created_at: DateTime<FixedOffset>,
    /// The paste's expiration date, if any.
    pub expires: Option<DateTime<FixedOffset>>,
    /// The paste's ID.
    pub id: String,
}

impl Termination for UserPaste {
    fn report(self) -> std::process::ExitCode {
        todo!()
    }
}
