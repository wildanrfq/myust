use std::{collections::HashMap, ops::FnOnce};

use super::{builders::*, structs::*, traits::traits::*, utils::utils::*};

use async_trait::async_trait;
use reqwest::{Method, Response};
use serde_json::{json, Value};

/// A client to interact with the API.
///
/// Use this if you're not doing anything users-related endpoints.
#[derive(Default)]
pub struct Client {
    inner: reqwest::Client,
}

impl Client {
    /// Instantiate a new Client.
    pub fn new() -> Self {
        Client {
            inner: reqwest::Client::new(),
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
    /// If you want to provide `expires` and `password`,
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
}

#[async_trait]
impl ClientPaste for Client {
    async fn request_create_paste(&self, json: Value) -> Response {
        self.request("PUT", ENDPOINT_URL.to_string() + "/paste", json)
            .await
    }

    async fn request_get_paste(&self, json: Value) -> Response {
        self.request("GET", ENDPOINT_URL.to_string() + "/paste", json)
            .await
    }
}
