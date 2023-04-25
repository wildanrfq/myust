#![cfg_attr(docsrs, doc(cfg(feature = "sync")))]

//! Synchronous implementation for clients.

use std::{collections::HashMap, ops::FnOnce};

use crate::{
    builders::*,
    structs::{response::MyustResponse, *},
    traits::*,
    utils::*,
};

use reqwest::Method;
use serde_json::{json, Value};

/// A synchronous authenticated client to interact with the API.
///
/// Use this if you're doing anything users-related.
#[derive(Default)]
pub struct SyncAuthClient {
    inner: reqwest::blocking::Client,
    token: String,
}

impl SyncAuthClient {
    fn check_token(client: reqwest::blocking::Client, token: &str) -> u16 {
        client
            .get(SELF_ENDPOINT)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .unwrap()
            .status()
            .as_u16()
    }

    fn request(&self, method: &str, url: &str, json: Value) -> MyustResponse {
        let methods = HashMap::from([
            ("GET", Method::GET),
            ("PUT", Method::PUT),
            ("DELETE", Method::DELETE),
        ]);
        let response = self
            .inner
            .request(methods[method].clone(), url.clone())
            .header("Authorization", self.token.clone())
            .json(&json)
            .send()
            .unwrap();
        let status_code = response.status().as_u16();
        let json_value = response.json::<Value>().ok();
        MyustResponse {
            json: json_value,
            status_code,
        }
    }

    /// Instantiate a new authenticated Client.
    ///
    /// Login to [mystb.in](https://mystb.in) to get your API token.
    ///
    /// Panics if the provided token is invalid.
    pub fn new(token: &str) -> Self {
        let client = reqwest::blocking::Client::new();
        let code = Self::check_token(client.clone(), token);
        match code {
            200 => SyncAuthClient {
                inner: client,
                token: format!("Bearer {}", token),
            },
            _ => panic!("The provided token is invalid."),
        }
    }

    /// Create a paste.
    pub fn create_paste<F>(&self, paste: F) -> Result<Paste, MystbinError>
    where
        F: FnOnce(&mut PasteBuilder) -> &mut PasteBuilder,
    {
        let mut builder = PasteBuilder {
            ..Default::default()
        };
        let data = paste(&mut builder);
        let expires = data.expires.map(|dt| dt.to_rfc3339());
        let files = vec![File {
            filename: data.filename.to_string(),
            content: data.content.to_string(),
        }];
        let json = json!({
            "files": files,
            "password": data.password,
            "expires": expires
        });
        let response = self.request_create_paste(json);

        match response.status_code {
            200 | 201 | 204 => {
                let paste_result = response.json.unwrap();
                Ok(Paste {
                    created_at: parse_date(paste_result["created_at"].as_str().unwrap()),
                    expires: data.expires,
                    files,
                    id: paste_result["id"].as_str().unwrap().to_string(),
                })
            }
            _ => {
                let data = response.json.unwrap();
                Err(MystbinError {
                    code: response.status_code,
                    error: data["error"].as_str().map(|s| s.to_string()),
                    notice: data["notice"].as_str().map(|s| s.to_string()),
                    detail: data["detail"]
                        .as_object()
                        .map(|m| m.clone().into_iter().collect()),
                })
            }
        }
    }

    /// Create a paste with multiple files.
    ///
    /// If you want to provide `expires` or `password`,
    /// put it in the first file.
    pub fn create_multifile_paste<F>(&self, pastes: F) -> Result<Paste, MystbinError>
    where
        F: FnOnce(&mut PastesBuilder) -> &mut PastesBuilder,
    {
        let mut builder = PastesBuilder::default();
        let data = &pastes(&mut builder).files;
        let expires = data[0].expires.map(|dt| dt.to_rfc3339());
        let first_paste = &data[0];
        let files = data
            .iter()
            .map(|file| File {
                filename: file.filename.clone(),
                content: file.content.clone(),
            })
            .collect();
        let json = json!({
            "files": files,
            "password": first_paste.password,
            "expires": expires
        });
        let response = self.request_create_paste(json);

        match response.status_code {
            200 | 201 | 204 => {
                let paste_result = response.json.unwrap();
                Ok(Paste {
                    created_at: parse_date(paste_result["created_at"].as_str().unwrap()),
                    expires: first_paste.expires,
                    files,
                    id: paste_result["id"].as_str().unwrap().to_string(),
                })
            }
            _ => {
                let data = response.json.unwrap();
                Err(MystbinError {
                    code: response.status_code,
                    error: data["error"].as_str().map(|s| s.to_string()),
                    notice: data["notice"].as_str().map(|s| s.to_string()),
                    detail: data["detail"]
                        .as_object()
                        .map(|m| m.clone().into_iter().collect()),
                })
            }
        }
    }

    /// Get a paste.
    pub fn get_paste<F>(&self, paste: F) -> Result<Paste, MystbinError>
    where
        F: FnOnce(&mut GetPasteBuilder) -> &mut GetPasteBuilder,
    {
        let mut builder = GetPasteBuilder::default();
        let data = paste(&mut builder);
        let response = self.request_get_paste(data.id.clone(), data.password.clone());
        match response.status_code {
            200 => {
                let paste_result = response.json.unwrap();
                let expires = if !paste_result["expires"].is_null() {
                    Some(parse_date(paste_result["expires"].as_str().unwrap()))
                } else {
                    None
                };
                let files = paste_result["files"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|x| File {
                        filename: x.get("filename").unwrap().to_string(),
                        content: x.get("content").unwrap().to_string(),
                    })
                    .collect::<Vec<File>>();
                Ok(Paste {
                    created_at: parse_date(paste_result["created_at"].as_str().unwrap()),
                    expires,
                    files,
                    id: data.id.clone(),
                })
            }
            _ => {
                let data = response.json.unwrap();
                Err(MystbinError {
                    code: response.status_code,
                    error: data["error"].as_str().map(|s| s.to_string()),
                    notice: data["notice"].as_str().map(|s| s.to_string()),
                    detail: data["detail"]
                        .as_object()
                        .map(|m| m.clone().into_iter().collect()),
                })
            }
        }
    }

    /// Delete a paste.
    pub fn delete_paste(&self, paste_id: &str) -> Result<DeleteResult, MystbinError> {
        let response = self.request_delete_paste(paste_id);
        match response.status_code {
            200 => Ok(DeleteResult {
                succeeded: Some(vec![paste_id.to_string()]),
                ..Default::default()
            }),
            _ => {
                let data = response.json.unwrap();
                Err(MystbinError {
                    code: response.status_code,
                    error: data["error"].as_str().map(|s| s.to_string()),
                    notice: data["notice"].as_str().map(|s| s.to_string()),
                    detail: data["detail"]
                        .as_object()
                        .map(|m| m.clone().into_iter().collect()),
                })
            }
        }
    }

    /// Delete pastes.
    pub fn delete_pastes(&self, paste_ids: Vec<&str>) -> Result<DeleteResult, MystbinError> {
        let json = json!({ "pastes": paste_ids });
        let response = self.request_delete_pastes(json);
        match response.status_code {
            200 => {
                let data = response.json.unwrap();
                Ok(DeleteResult {
                    succeeded: Some(
                        data["succeeded"]
                            .as_array()
                            .unwrap()
                            .iter()
                            .map(|p| p.to_string())
                            .collect(),
                    ),
                    failed: Some(
                        data["failed"]
                            .as_array()
                            .unwrap()
                            .iter()
                            .map(|p| p.to_string())
                            .collect(),
                    ),
                })
            }
            _ => {
                let data = response.json.unwrap();
                Err(MystbinError {
                    code: response.status_code,
                    error: data["error"].as_str().map(|s| s.to_string()),
                    notice: data["notice"].as_str().map(|s| s.to_string()),
                    detail: data["detail"]
                        .as_object()
                        .map(|m| m.clone().into_iter().collect()),
                })
            }
        }
    }

    /// Get the authenticated user pastes.
    pub fn get_user_pastes<F>(&self, options: F) -> Result<Vec<UserPaste>, MystbinError>
    where
        F: FnOnce(&mut UserPastesOptions) -> &mut UserPastesOptions,
    {
        let mut builder = UserPastesOptions::default();
        let data = options(&mut builder);
        let json = json!({
            "limit": data.limit,
            "page": data.page
        });
        let response = self.request_get_user_pastes(json);
        match response.status_code {
            200 => {
                let results = response.json.unwrap();
                let pastes = results["pastes"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|result| {
                        let expires = result["expires"].as_str().map(parse_date);
                        UserPaste {
                            created_at: parse_date(result["created_at"].as_str().unwrap()),
                            expires,
                            id: result["id"].as_str().unwrap().to_string(),
                        }
                    })
                    .collect();
                Ok(pastes)
            }
            _ => {
                let data = response.json.unwrap();
                Err(MystbinError {
                    code: response.status_code,
                    error: data["error"].as_str().map(|s| s.to_string()),
                    notice: data["notice"].as_str().map(|s| s.to_string()),
                    detail: data["detail"]
                        .as_object()
                        .map(|m| m.clone().into_iter().collect()),
                })
            }
        }
    }

    /// Add a paste to the authenticated user's bookmark.
    pub fn create_bookmark(&self, paste_id: &str) -> Result<(), MystbinError> {
        let json = json!({ "paste_id": paste_id });
        let response = self.request_create_bookmark(json);
        match response.status_code {
            201 => Ok(()),
            _ => {
                let data = response.json.unwrap();
                Err(MystbinError {
                    code: response.status_code,
                    error: data["error"].as_str().map(|s| s.to_string()),
                    notice: data["notice"].as_str().map(|s| s.to_string()),
                    detail: data["detail"]
                        .as_object()
                        .map(|m| m.clone().into_iter().collect()),
                })
            }
        }
    }

    /// Delete a paste from the authenticated user's bookmark.
    pub fn delete_bookmark(&self, paste_id: &str) -> Result<(), MystbinError> {
        let json = json!({ "paste_id": paste_id });
        let response = self.request_delete_bookmark(json);
        match response.status_code {
            204 => Ok(()),
            _ => {
                let data = response.json.unwrap();
                Err(MystbinError {
                    code: response.status_code,
                    error: data["error"].as_str().map(|s| s.to_string()),
                    notice: data["notice"].as_str().map(|s| s.to_string()),
                    detail: data["detail"]
                        .as_object()
                        .map(|m| m.clone().into_iter().collect()),
                })
            }
        }
    }

    /// Get the authenticated user's bookmarks.
    pub fn get_user_bookmarks(&self) -> Result<Vec<UserPaste>, MystbinError> {
        let response = self.request_get_user_bookmarks();
        match response.status_code {
            200 => {
                let data = response.json.unwrap();
                let bookmarks = data["bookmarks"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|paste| {
                        let expires = paste["expires"].as_str().map(parse_date);
                        UserPaste {
                            created_at: parse_date(paste["created_at"].as_str().unwrap()),
                            expires,
                            id: paste["id"].as_str().unwrap().to_string(),
                        }
                    })
                    .collect();
                Ok(bookmarks)
            }
            _ => {
                let data = response.json.unwrap();
                Err(MystbinError {
                    code: response.status_code,
                    error: data["error"].as_str().map(|s| s.to_string()),
                    notice: data["notice"].as_str().map(|s| s.to_string()),
                    detail: data["detail"]
                        .as_object()
                        .map(|m| m.clone().into_iter().collect()),
                })
            }
        }
    }
}

impl SyncAuthClientPaste for SyncAuthClient {
    fn request_create_paste(&self, json: Value) -> MyustResponse {
        self.request("PUT", PASTE_ENDPOINT, json)
    }

    fn request_delete_paste(&self, paste_id: &str) -> MyustResponse {
        self.request(
            "DELETE",
            &format!("{}/{}", PASTE_ENDPOINT, paste_id),
            json!({}),
        )
    }

    fn request_delete_pastes(&self, json: Value) -> MyustResponse {
        self.request("DELETE", PASTE_ENDPOINT, json)
    }

    fn request_get_paste(&self, paste_id: String, password: Option<String>) -> MyustResponse {
        let url = if password.is_some() {
            format!(
                "{}/{}?password={}",
                PASTE_ENDPOINT,
                paste_id,
                password.unwrap()
            )
        } else {
            format!("{}/{}", PASTE_ENDPOINT, paste_id)
        };
        self.request("GET", &url, json!({}))
    }

    fn request_get_user_pastes(&self, json: Value) -> MyustResponse {
        self.request("GET", USER_PASTES_ENDPOINT, json)
    }
}

impl SyncAuthClientBookmark for SyncAuthClient {
    fn request_create_bookmark(&self, json: Value) -> MyustResponse {
        self.request("PUT", BOOKMARK_ENDPOINT, json)
    }

    fn request_delete_bookmark(&self, json: Value) -> MyustResponse {
        self.request("DELETE", BOOKMARK_ENDPOINT, json)
    }

    fn request_get_user_bookmarks(&self) -> MyustResponse {
        self.request("GET", BOOKMARK_ENDPOINT, json!({}))
    }
}

/// A synchronous client to interact with the API.
///
/// Use this if you're not doing anything users-related endpoints.
#[derive(Default)]
pub struct SyncClient {
    inner: reqwest::blocking::Client,
}

impl SyncClient {
    /// Instantiate a new Client.
    pub fn new() -> Self {
        SyncClient {
            inner: reqwest::blocking::Client::new(),
        }
    }

    fn request(&self, method: &str, url: &str, json: Value) -> MyustResponse {
        let methods = HashMap::from([
            ("GET", Method::GET),
            ("PUT", Method::PUT),
            ("DELETE", Method::DELETE),
        ]);
        let response = self
            .inner
            .request(methods[method].clone(), url.clone())
            .json(&json)
            .send()
            .unwrap();
        let status_code = response.status().as_u16();
        let json_value = response.json::<Value>().ok();
        MyustResponse {
            json: json_value,
            status_code,
        }
    }

    /// Create a paste.
    pub fn create_paste<F>(&self, paste: F) -> Result<Paste, MystbinError>
    where
        F: FnOnce(&mut PasteBuilder) -> &mut PasteBuilder,
    {
        let mut builder = PasteBuilder {
            ..Default::default()
        };
        let data = paste(&mut builder);
        let expires = data.expires.map(|dt| dt.to_rfc3339());
        let files = vec![File {
            filename: data.filename.to_string(),
            content: data.content.to_string(),
        }];
        let json = json!({
            "files": files,
            "password": data.password,
            "expires": expires
        });
        let response = self.request_create_paste(json);

        match response.status_code {
            200 | 201 | 204 => {
                let paste_result = response.json.unwrap();
                Ok(Paste {
                    created_at: parse_date(paste_result["created_at"].as_str().unwrap()),
                    expires: data.expires,
                    files,
                    id: paste_result["id"].as_str().unwrap().to_string(),
                })
            }
            _ => {
                let data = response.json.unwrap();
                Err(MystbinError {
                    code: response.status_code,
                    error: data["error"].as_str().map(|s| s.to_string()),
                    notice: data["notice"].as_str().map(|s| s.to_string()),
                    detail: data["detail"]
                        .as_object()
                        .map(|m| m.clone().into_iter().collect()),
                })
            }
        }
    }

    /// Create a paste with multiple files.
    ///
    /// If you want to provide `expires` and `password`,
    /// put it in the first file.
    pub fn create_multifile_paste<F>(&self, pastes: F) -> Result<Paste, MystbinError>
    where
        F: FnOnce(&mut PastesBuilder) -> &mut PastesBuilder,
    {
        let mut builder = PastesBuilder::default();
        let data = &pastes(&mut builder).files;
        let expires = data[0].expires.map(|dt| dt.to_rfc3339());
        let first_paste = &data[0];
        let files = data
            .iter()
            .map(|file| File {
                filename: file.filename.clone(),
                content: file.content.clone(),
            })
            .collect();

        let json = json!({
            "files": files,
            "password": first_paste.password,
            "expires": expires
        });
        let response = self.request_create_paste(json);

        match response.status_code {
            200 | 201 | 204 => {
                let paste_result = response.json.unwrap();
                Ok(Paste {
                    created_at: parse_date(paste_result["created_at"].as_str().unwrap()),
                    expires: first_paste.expires,
                    files,
                    id: paste_result["id"].as_str().unwrap().to_string(),
                })
            }
            _ => {
                let data = response.json.unwrap();
                Err(MystbinError {
                    code: response.status_code,
                    error: data["error"].as_str().map(|s| s.to_string()),
                    notice: data["notice"].as_str().map(|s| s.to_string()),
                    detail: data["detail"]
                        .as_object()
                        .map(|m| m.clone().into_iter().collect()),
                })
            }
        }
    }

    /// Get a paste.
    pub fn get_paste<F>(&self, paste: F) -> Result<Paste, MystbinError>
    where
        F: FnOnce(&mut GetPasteBuilder) -> &mut GetPasteBuilder,
    {
        let mut builder = GetPasteBuilder::default();
        let data = paste(&mut builder);
        let response = self.request_get_paste(data.id.clone(), data.password.clone());
        match response.status_code {
            200 => {
                let paste_result = response.json.unwrap();
                let expires = if !paste_result["expires"].is_null() {
                    Some(parse_date(paste_result["expires"].as_str().unwrap()))
                } else {
                    None
                };
                let files = paste_result["files"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|x| File {
                        filename: x.get("filename").unwrap().to_string(),
                        content: x.get("content").unwrap().to_string(),
                    })
                    .collect::<Vec<File>>();
                Ok(Paste {
                    created_at: parse_date(paste_result["created_at"].as_str().unwrap()),
                    expires,
                    files,
                    id: data.id.clone(),
                })
            }
            _ => {
                let data = response.json.unwrap();
                Err(MystbinError {
                    code: response.status_code,
                    error: data["error"].as_str().map(|s| s.to_string()),
                    notice: data["notice"].as_str().map(|s| s.to_string()),
                    detail: data["detail"]
                        .as_object()
                        .map(|m| m.clone().into_iter().collect()),
                })
            }
        }
    }
}

impl SyncClientPaste for SyncClient {
    fn request_create_paste(&self, json: Value) -> MyustResponse {
        self.request("PUT", PASTE_ENDPOINT, json)
    }

    fn request_get_paste(&self, paste_id: String, password: Option<String>) -> MyustResponse {
        let url = if password.is_some() {
            format!(
                "{}/{}?password={}",
                PASTE_ENDPOINT,
                paste_id,
                password.unwrap()
            )
        } else {
            format!("{}/{}", PASTE_ENDPOINT, paste_id)
        };
        self.request("GET", &url, json!({}))
    }
}
