use std::{collections::HashMap, ops::FnOnce};

use super::{builders::*, structs::*, traits::traits::*, utils::*};

use async_trait::async_trait;
use reqwest::{Method, Response};
use serde_json::{json, Value};

/// An authenticated client to interact with the API.
///
/// Use this if you're doing anything users-related.
#[derive(Default)]
pub struct AuthClient {
    inner: reqwest::Client,
    token: String,
}

impl AuthClient {
    async fn check_token(client: reqwest::Client, token: &str) -> u16 {
        client
            .get(ENDPOINT_URL.to_string() + "/users/@me")
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await
            .unwrap()
            .status()
            .as_u16()
    }

    /// Instantiate a new authenticated Client.
    ///
    /// Login to <https://mystb.in> to get your API token.
    /// 
    /// Panics if the provided token is invalid.
    pub async fn new(token: &str) -> Self {
        let client = reqwest::Client::new();
        let code = Self::check_token(client.clone(), token).await;
        match code {
            200 => AuthClient {
                inner: client,
                token: format!("Bearer {}", token),
            },
            _ => panic!("The provided token is invalid."),
        }
    }

    async fn request(&self, method: &str, url: String, json: Value) -> Response {
        let methods = HashMap::from([
            ("GET", Method::GET),
            ("PUT", Method::PUT),
            ("DELETE", Method::DELETE),
        ]);
        self.inner
            .request(methods[method].clone(), url.clone())
            .header("Authorization", self.token.clone())
            .json(&json)
            .send()
            .await
            .unwrap()
    }

    /// Create a paste.
    pub async fn create_paste<F>(&self, paste: F) -> Result<Paste, MystbinError>
    where
        F: FnOnce(&mut PasteBuilder) -> &mut PasteBuilder,
    {
        let mut builder = PasteBuilder {
            ..Default::default()
        };
        let data = paste(&mut builder);
        let expires = if data.expires.is_some() {
            Some(data.expires.unwrap().to_rfc3339())
        } else {
            None
        };
        let files = vec![File {
            filename: data.filename.to_string(),
            content: data.content.to_string(),
        }];
        let json = json!({
            "files": files,
            "password": data.password,
            "expires": expires
        });
        let response = self.request_create_paste(json).await;

        match response.status().as_u16() {
            200 | 201 | 204 => {
                let paste_result = response.json::<Value>().await.unwrap();
                Ok(Paste {
                    created_at: parse_date(paste_result["created_at"].as_str().unwrap()),
                    expires: data.expires,
                    files: files,
                    id: paste_result["id"].as_str().unwrap().to_string(),
                })
            }
            status_code => Err(MystbinError {
                code: status_code,
                ..Default::default()
            }),
        }
    }

    /// Create a paste with multiple files.
    ///
    /// If you want to provide `expires` or `password`,
    /// put it in the first file.
    pub async fn create_multifile_paste<F>(&self, pastes: F) -> Result<Paste, MystbinError>
    where
        F: FnOnce(&mut PastesBuilder) -> &mut PastesBuilder,
    {
        let mut builder = PastesBuilder::default();
        let data = &pastes(&mut builder).files;
        let expires = if data[0].expires.is_some() {
            Some(data[0].expires.unwrap().to_rfc3339())
        } else {
            None
        };
        let mut files = vec![];
        let first_paste = &data[0];
        for file in data {
            files.push(File {
                filename: file.filename.clone(),
                content: file.content.clone(),
            })
        }
        let json = json!({
            "files": files,
            "password": first_paste.password,
            "expires": expires
        });
        let response = self.request_create_paste(json).await;

        match response.status().as_u16() {
            200 | 201 | 204 => {
                let paste_result = response.json::<Value>().await.unwrap();
                Ok(Paste {
                    created_at: parse_date(paste_result["created_at"].as_str().unwrap()),
                    expires: first_paste.expires,
                    files: files,
                    id: paste_result["id"].as_str().unwrap().to_string(),
                })
            }
            status_code => Err(MystbinError {
                code: status_code,
                ..Default::default()
            }),
        }
    }

    /// Get a paste.
    pub async fn get_paste<F>(&self, paste: F) -> Result<Paste, MystbinError>
    where
        F: FnOnce(&mut GetPasteBuilder) -> &mut GetPasteBuilder,
    {
        let mut builder = GetPasteBuilder::default();
        let data = paste(&mut builder);
        let json = json!({
            "paste_id": data.id,
            "password": data.password
        });
        let response = self.request_get_paste(json).await;
        match response.status().as_u16() {
            200 => {
                let paste_result = response.json::<Value>().await.unwrap();
                let expires = if !paste_result["expires"].is_null() {
                    Some(parse_date(paste_result["expires"].as_str().unwrap()))
                } else {
                    None
                };
                let files = paste_result["files"]
                    .as_array()
                    .unwrap()
                    .into_iter()
                    .map(|x| File {
                        filename: x.get("filename").unwrap().to_string(),
                        content: x.get("content").unwrap().to_string(),
                    })
                    .collect::<Vec<File>>();
                Ok(Paste {
                    created_at: parse_date(paste_result["created_at"].as_str().unwrap()),
                    expires: expires,
                    files: files,
                    id: data.id.clone(),
                })
            }
            status_code => Err(MystbinError {
                code: status_code,
                ..Default::default()
            }),
        }
    }

    /// Delete a paste.
    pub async fn delete_paste(&self, paste_id: &str) -> Result<DeleteResult, MystbinError> {
        let response = self.request_delete_paste(paste_id).await;
        match response.status().as_u16() {
            200 => Ok(DeleteResult {
                succeeded: Some(vec![paste_id.to_string()]),
                ..Default::default()
            }),
            _ => {
                return Err(MystbinError {
                    code: response.status().as_u16(),
                });
            }
        }
    }

    /// Delete pastes.
    pub async fn delete_pastes(&self, paste_ids: Vec<&str>) -> Result<DeleteResult, MystbinError> {
        let json = json!({ "pastes": paste_ids });
        let response = self.request_delete_pastes(json).await;
        match response.status().as_u16() {
            200 => {
                let data = response.json::<Value>().await.unwrap();
                Ok(DeleteResult {
                    succeeded: Some(
                        data["succeeded"]
                            .as_array()
                            .unwrap()
                            .into_iter()
                            .map(|p| p.to_string())
                            .collect(),
                    ),
                    failed: Some(
                        data["failed"]
                            .as_array()
                            .unwrap()
                            .into_iter()
                            .map(|p| p.to_string())
                            .collect(),
                    ),
                })
            }
            _ => Err(MystbinError {
                code: response.status().as_u16(),
                ..Default::default()
            }),
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
        match response.status().as_u16() {
            200 => {
                let mut pastes = vec![];
                let results = response.json::<Value>().await.unwrap();
                for result in results["pastes"].as_array().unwrap() {
                    let expires = if !result["expires"].is_null() {
                        Some(parse_date(result["expires"].as_str().unwrap()))
                    } else {
                        None
                    };
                    pastes.push(UserPaste {
                        created_at: parse_date(result["created_at"].as_str().unwrap()),
                        expires: expires,
                        id: result["id"].as_str().unwrap().to_string(),
                    })
                }
                Ok(pastes)
            }
            _ => Err(MystbinError {
                code: response.status().as_u16(),
                ..Default::default()
            }),
        }
    }

    /// Add a paste to the authenticated user's bookmark.
    pub async fn create_bookmark(&self, paste_id: &str) -> Result<(), MystbinError> {
        let json = json!({ "paste_id": paste_id });
        let response = self.request_create_bookmark(json).await;
        match response.status().as_u16() {
            201 => Ok(()),
            _ => Err(MystbinError {
                code: response.status().as_u16(),
                ..Default::default()
            }),
        }
    }

    /// Delete a paste from the authenticated user's bookmark.
    pub async fn delete_bookmark(&self, paste_id: &str) -> Result<(), MystbinError> {
        let json = json!({ "paste_id": paste_id });
        let response = self.request_delete_bookmark(json).await;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(MystbinError {
                code: response.status().as_u16(),
                ..Default::default()
            }),
        }
    }

    /// Get the authenticated user's bookmarks.
    pub async fn get_user_bookmarks(&self) -> Result<Vec<UserPaste>, MystbinError> {
        let response = self.request_get_user_bookmarks().await;
        match response.status().as_u16() {
            200 => {
                let data = response.json::<Value>().await.unwrap();
                let mut bookmarks = vec![];
                for paste in data["bookmarks"].as_array().unwrap() {
                    let expires = if !paste["expires"].is_null() {
                        Some(parse_date(paste["expires"].as_str().unwrap()))
                    } else {
                        None
                    };
                    bookmarks.push(UserPaste {
                        created_at: parse_date(paste["created_at"].as_str().unwrap()),
                        expires: expires,
                        id: paste["id"].as_str().unwrap().to_string(),
                    })
                }
                Ok(bookmarks)
            }
            _ => Err(MystbinError {
                code: response.status().as_u16(),
                ..Default::default()
            }),
        }
    }
}

#[async_trait]
impl AuthClientPaste for AuthClient {
    async fn request_create_paste(&self, json: Value) -> Response {
        self.request("PUT", ENDPOINT_URL.to_string() + "/paste", json)
            .await
    }

    async fn request_delete_paste(&self, paste_id: &str) -> Response {
        self.request(
            "DELETE",
            ENDPOINT_URL.to_string() + "/paste/" + paste_id,
            json!({}),
        )
        .await
    }

    async fn request_delete_pastes(&self, json: Value) -> Response {
        self.request("DELETE", ENDPOINT_URL.to_string() + "/paste", json)
            .await
    }

    async fn request_get_paste(&self, json: Value) -> Response {
        self.request("GET", ENDPOINT_URL.to_string() + "/paste", json)
            .await
    }

    async fn request_get_user_pastes(&self, json: Value) -> Response {
        self.request("GET", ENDPOINT_URL.to_string() + "/pastes/@me", json)
            .await
    }
}

#[async_trait]
impl AuthClientBookmark for AuthClient {
    async fn request_create_bookmark(&self, json: Value) -> Response {
        self.request("PUT", ENDPOINT_URL.to_string() + "/users/bookmarks", json)
            .await
    }

    async fn request_delete_bookmark(&self, json: Value) -> Response {
        self.request(
            "DELETE",
            ENDPOINT_URL.to_string() + "/users/bookmarks",
            json,
        )
        .await
    }

    async fn request_get_user_bookmarks(&self) -> Response {
        self.request(
            "GET",
            ENDPOINT_URL.to_string() + "/users/bookmarks",
            json!({}),
        )
        .await
    }
}

