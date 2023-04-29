use std::{collections::HashMap, ops::FnOnce};

use crate::{
    builders::*,
    structs::{response::MyustResponse, *},
    traits::*,
    utils::*,
};

use async_trait::async_trait;
use reqwest::Method;
use serde_json::{json, Map, Value};

/// A client to interact with the API.
///
/// Use this if you're not doing anything users-related endpoints.
#[derive(Default)]
pub struct Client {
    inner: reqwest::Client,
    token: Option<String>,
}

impl Client {
    async fn check_token(client: reqwest::Client, token: String) -> u16 {
        client
            .get(SELF_ENDPOINT)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await
            .unwrap()
            .status()
            .as_u16()
    }

    /// Instantiate a new Client.
    pub fn new() -> Self {
        Client {
            inner: reqwest::Client::new(),
            ..Default::default()
        }
    }

    pub async fn auth(mut self, token: impl Into<String>) -> Self {
        let token_str = token.into();
        let code = Self::check_token(self.inner.clone(), token_str.clone()).await;
        match code {
            200 => {
                self.token = Some(format!("Bearer {}", token_str));
                self
            }
            _ => panic!("the provided token is invalid"),
        }
    }

    async fn request(&self, method: &str, url: &str, json: Value) -> MyustResponse {
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
                .await
                .unwrap()
        } else {
            self.inner
                .request(methods[method].clone(), url.clone())
                .json(&json)
                .send()
                .await
                .unwrap()
        };
        let status_code = response.status().as_u16();
        let json_value = response.json::<Value>().await.ok();
        MyustResponse {
            json: json_value,
            status_code,
        }
    }

    /// Create a paste.
    pub async fn create_paste<F>(&self, paste: F) -> Result<PasteResult, MystbinError>
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
        if let Some(expiry) = &data.expires {
            if expiry.valid() {
                map.insert("expires".to_string(), json!(expiry.to_rfc3339()));
            } else {
                let invalid = expiry.invalid_field();
                panic!("{} can not be negative, value: {}", invalid.0, invalid.1)
            }
        };
        let json = Value::Object(map);
        let response = self.request_create_paste(json).await;

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
    pub async fn create_multifile_paste<F>(&self, pastes: F) -> Result<PasteResult, MystbinError>
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
                map.insert("expires".to_string(), json!(expiry.to_rfc3339()));
            } else {
                let invalid = expiry.invalid_field();
                panic!("{} can not be negative, value: {}", invalid.0, invalid.1)
            }
        };
        let json = Value::Object(map);
        let response = self.request_create_paste(json).await;

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
    pub async fn get_paste<F>(&self, paste: F) -> Result<PasteResult, MystbinError>
    where
        F: FnOnce(&mut GetPasteBuilder) -> &mut GetPasteBuilder,
    {
        let mut builder = GetPasteBuilder::default();
        let data = paste(&mut builder);
        let response = self
            .request_get_paste(data.id.clone(), data.password.clone())
            .await;
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

    /// Delete a paste.
    pub async fn delete_paste(&self, paste_id: &str) -> Result<DeleteResult, MystbinError> {
        let response = self.request_delete_paste(paste_id).await;
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
    pub async fn delete_pastes(&self, paste_ids: Vec<&str>) -> Result<DeleteResult, MystbinError> {
        let json = json!({ "pastes": paste_ids });
        let response = self.request_delete_pastes(json).await;
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
    pub async fn get_user_pastes<F>(&self, options: F) -> Result<Vec<UserPaste>, MystbinError>
    where
        F: FnOnce(&mut UserPastesOptions) -> &mut UserPastesOptions,
    {
        let mut builder = UserPastesOptions::default();
        let data = options(&mut builder);
        let json = json!({
            "limit": data.limit,
            "page": data.page
        });
        let response = self.request_get_user_pastes(json).await;
        match response.status_code {
            200 => {
                let results = response.json.unwrap();
                let pastes = results["pastes"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|result| UserPaste {
                        created_at: result["created_at"].as_str().unwrap().to_string(),
                        expires: result["expires"].as_str().map(|d| d.to_string()),
                        id: result["id"].as_str().unwrap().to_string(),
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
    pub async fn create_bookmark(&self, paste_id: &str) -> Result<(), MystbinError> {
        let json = json!({ "paste_id": paste_id });
        let response = self.request_create_bookmark(json).await;
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
    pub async fn delete_bookmark(&self, paste_id: &str) -> Result<(), MystbinError> {
        let json = json!({ "paste_id": paste_id });
        let response = self.request_delete_bookmark(json).await;
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
    pub async fn get_user_bookmarks(&self) -> Result<Vec<UserPaste>, MystbinError> {
        let response = self.request_get_user_bookmarks().await;
        match response.status_code {
            200 => {
                let data = response.json.unwrap();
                let bookmarks = data["bookmarks"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|paste| UserPaste {
                        created_at: paste["created_at"].as_str().unwrap().to_string(),
                        expires: paste["expires"].as_str().map(|d| d.to_string()),
                        id: paste["id"].as_str().unwrap().to_string(),
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

#[async_trait]
impl ClientPaste for Client {
    async fn request_create_paste(&self, json: Value) -> MyustResponse {
        self.request("PUT", PASTE_ENDPOINT, json).await
    }

    async fn request_delete_paste(&self, paste_id: &str) -> MyustResponse {
        self.request(
            "DELETE",
            &format!("{}/{}", PASTE_ENDPOINT, paste_id),
            json!({}),
        )
        .await
    }

    async fn request_delete_pastes(&self, json: Value) -> MyustResponse {
        self.request("DELETE", PASTE_ENDPOINT, json).await
    }

    async fn request_get_paste(&self, paste_id: String, password: Option<String>) -> MyustResponse {
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
        self.request("GET", &url, json!({})).await
    }

    async fn request_get_user_pastes(&self, json: Value) -> MyustResponse {
        self.request("GET", USER_PASTES_ENDPOINT, json).await
    }
}

#[async_trait]
impl ClientBookmark for Client {
    async fn request_create_bookmark(&self, json: Value) -> MyustResponse {
        self.request("PUT", BOOKMARK_ENDPOINT, json).await
    }

    async fn request_delete_bookmark(&self, json: Value) -> MyustResponse {
        self.request("DELETE", BOOKMARK_ENDPOINT, json).await
    }

    async fn request_get_user_bookmarks(&self) -> MyustResponse {
        self.request("GET", BOOKMARK_ENDPOINT, json!({})).await
    }
}
