use crate::structs::response::MyustResponse;
use async_trait::async_trait;
use serde_json::Value;

#[async_trait]
pub trait AuthClientPaste {
    async fn request_create_paste(&self, json: Value) -> MyustResponse;
    async fn request_delete_paste(&self, paste_id: &str) -> MyustResponse;
    async fn request_delete_pastes(&self, json: Value) -> MyustResponse;
    async fn request_get_paste(&self, paste_id: String, password: Option<String>) -> MyustResponse;
    async fn request_get_user_pastes(&self, json: Value) -> MyustResponse;
}

#[async_trait]
pub trait AuthClientBookmark {
    async fn request_create_bookmark(&self, json: Value) -> MyustResponse;
    async fn request_delete_bookmark(&self, json: Value) -> MyustResponse;
    async fn request_get_user_bookmarks(&self) -> MyustResponse;
}

#[async_trait]
pub trait ClientPaste {
    async fn request_create_paste(&self, json: Value) -> MyustResponse;
    async fn request_get_paste(&self, paste_id: String, password: Option<String>) -> MyustResponse;
}
pub trait SyncAuthClientPaste {
    fn request_create_paste(&self, json: Value) -> MyustResponse;
    fn request_delete_paste(&self, paste_id: &str) -> MyustResponse;
    fn request_delete_pastes(&self, json: Value) -> MyustResponse;
    fn request_get_paste(&self, paste_id: String, password: Option<String>) -> MyustResponse;
    fn request_get_user_pastes(&self, json: Value) -> MyustResponse;
}

pub trait SyncAuthClientBookmark {
    fn request_create_bookmark(&self, json: Value) -> MyustResponse;
    fn request_delete_bookmark(&self, json: Value) -> MyustResponse;
    fn request_get_user_bookmarks(&self) -> MyustResponse;
}

pub trait SyncClientPaste {
    fn request_create_paste(&self, json: Value) -> MyustResponse;
    fn request_get_paste(&self, paste_id: String, password: Option<String>) -> MyustResponse;
}
