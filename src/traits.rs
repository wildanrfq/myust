pub(super) mod traits {
    use async_trait::async_trait;
    use reqwest::{Response, blocking::Response as BlockingResponse};
    use serde_json::Value;

    #[async_trait]
    pub trait AuthClientPaste {
        async fn request_create_paste(&self, json: Value) -> Response;
        async fn request_delete_paste(&self, paste_id: &str) -> Response;
        async fn request_delete_pastes(&self, json: Value) -> Response;
        async fn request_get_paste(&self, json: Value) -> Response;
        async fn request_get_user_pastes(&self, json: Value) -> Response;
    }

    #[async_trait]
    pub trait AuthClientBookmark {
        async fn request_create_bookmark(&self, json: Value) -> Response;
        async fn request_delete_bookmark(&self, json: Value) -> Response;
        async fn request_get_user_bookmarks(&self) -> Response;
    }

    #[async_trait]
    pub trait ClientPaste {
        async fn request_create_paste(&self, json: Value) -> Response;
        async fn request_get_paste(&self, json: Value) -> Response;
    }
    pub trait SyncAuthClientPaste {
        fn request_create_paste(&self, json: Value) -> BlockingResponse;
        fn request_delete_paste(&self, paste_id: &str) -> BlockingResponse;
        fn request_delete_pastes(&self, json: Value) -> BlockingResponse;
        fn request_get_paste(&self, json: Value) -> BlockingResponse;
        fn request_get_user_pastes(&self, json: Value) -> BlockingResponse;
    }

    pub trait SyncAuthClientBookmark {
        fn request_create_bookmark(&self, json: Value) -> BlockingResponse;
        fn request_delete_bookmark(&self, json: Value) -> BlockingResponse;
        fn request_get_user_bookmarks(&self) -> BlockingResponse;
    }

    pub trait SyncClientPaste {
        fn request_create_paste(&self, json: Value) -> BlockingResponse;
        fn request_get_paste(&self, json: Value) -> BlockingResponse;
    }
}
