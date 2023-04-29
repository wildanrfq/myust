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
use serde_json::{json, Map, Value};

/// A synchronous client to interact with the API.
///
/// Use this if you're not doing anything users-related endpoints.
#[derive(Default)]
pub struct SyncClient {
    inner: reqwest::blocking::Client,
    token: Option<String>,
}

impl SyncClient {
    fn check_token(client: reqwest::blocking::Client, token: String) -> u16 {
        client
            .get(SELF_ENDPOINT)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .unwrap()
            .status()
            .as_u16()
    }

    /// Instantiate a new Client.
    pub fn new() -> Self {
        SyncClient {
            inner: reqwest::blocking::Client::new(),
            ..Default::default()
        }
    }

    /// Authenticate to mystb.in's API.
    /// 
    /// This method will panic if the provided token is invalid.
    pub fn auth(mut self, token: impl Into<String>) -> Self {
        let token_str = token.into();
        let code = Self::check_token(self.inner.clone(), token_str.clone());
        match code {
            200 => {
                self.token = Some(format!("Bearer {}", token_str));
                self
            }
            _ => panic!("the provided token is invalid"),
        }
    }

    fn request(&self, method: &str, url: &str, json: Value) -> MyustResponse {
        let methods = HashMap::from([
            ("GET", Method::GET),
            ("PUT", Method::PUT),
            ("DELETE", Method::DELETE),
        ]);
        let response = if let Some(token) = &self.token {
            self.inner
                .request(methods[method].clone(), url.clone())
                .header("Authorization", token)
                .json(&json)
                .send()
                .unwrap()
        } else {
            self.inner
                .request(methods[method].clone(), url.clone())
                .json(&json)
                .send()
                .unwrap()
        };
        let status_code = response.status().as_u16();
        let json_value = response.json::<Value>().ok();
        MyustResponse {
            json: json_value,
            status_code,
        }
    }

    /// Create a paste.
    pub fn create_paste<F>(&self, paste: F) -> Result<PasteResult, MystbinError>
    where
        F: FnOnce(&mut PasteBuilder) -> &mut PasteBuilder,
    {
        let mut builder = PasteBuilder {
            ..Default::default()
        };
        let data = paste(&mut builder);
        let files = vec![File {
            filename: data.filename.to_string(),
            content: data.content.to_string(),
        }];
        let mut map = Map::new();
        map.insert("files".to_string(), json!(files));
        map.insert("password".to_string(), json!(data.password));
        println!("{:#?}", data.expires);
        if let Some(expiry) = &data.expires {
            if expiry.valid() {
                if expiry.is_default() {
                    map.insert("expires".to_string(), json!(None::<()>));
                } else {
                    map.insert("expires".to_string(), json!(expiry.to_rfc3339()));
                }
            } else {
                let invalid = expiry.invalid_field();
                panic!("{} can not be negative, value: {}", invalid.0, invalid.1)
            }
        };
        let json = Value::Object(map);
        let response = self.request_create_paste(json);

        match response.status_code {
            200 | 201 | 204 => {
                let paste_result = response.json.unwrap();
                Ok(PasteResult {
                    created_at: paste_result["created_at"].as_str().unwrap().to_string(),
                    expires: paste_result["expires"].as_str().map(|d| d.to_string()),
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
    pub fn create_multifile_paste<F>(&self, pastes: F) -> Result<PasteResult, MystbinError>
    where
        F: FnOnce(&mut PastesBuilder) -> &mut PastesBuilder,
    {
        let mut builder = PastesBuilder::default();
        let data = &pastes(&mut builder).files;
        let first_paste = &data[0];
        let files = data
            .iter()
            .map(|file| File {
                filename: file.filename.clone(),
                content: file.content.clone(),
            })
            .collect();

        let mut map = Map::new();
        map.insert("files".to_string(), json!(files));
        map.insert("password".to_string(), json!(first_paste.password));
        if let Some(expiry) = &first_paste.expires {
            if expiry.valid() {
                if expiry.is_default() {
                    map.insert("expires".to_string(), json!(None::<()>));
                } else {
                    map.insert("expires".to_string(), json!(expiry.to_rfc3339()));
                }
            } else {
                let invalid = expiry.invalid_field();
                panic!("{} can not be negative, value: {}", invalid.0, invalid.1)
            }
        };
        let json = Value::Object(map);
        let response = self.request_create_paste(json);

        match response.status_code {
            200 | 201 | 204 => {
                let paste_result = response.json.unwrap();
                Ok(PasteResult {
                    created_at: paste_result["created_at"].as_str().unwrap().to_string(),
                    expires: paste_result["expires"].as_str().map(|d| d.to_string()),
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
    pub fn get_paste<F>(&self, paste: F) -> Result<PasteResult, MystbinError>
    where
        F: FnOnce(&mut GetPasteBuilder) -> &mut GetPasteBuilder,
    {
        let mut builder = GetPasteBuilder::default();
        let data = paste(&mut builder);
        let response = self.request_get_paste(data.id.clone(), data.password.clone());
        match response.status_code {
            200 => {
                let paste_result = response.json.unwrap();
                let files = paste_result["files"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|x| File {
                        filename: x.get("filename").unwrap().to_string(),
                        content: x.get("content").unwrap().to_string(),
                    })
                    .collect::<Vec<File>>();
                Ok(PasteResult {
                    created_at: paste_result["created_at"].as_str().unwrap().to_string(),
                    expires: paste_result["expires"].as_str().map(|d| d.to_string()),
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

impl SyncClientBookmark for SyncClient {
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
