use std::mem::take;

use crate::Expiry;

/// The builder to get a paste.
#[derive(Debug, Default)]
pub struct GetPasteBuilder {
    pub id: String,
    pub password: Option<String>,
}

impl GetPasteBuilder {
    /// The ID of the paste.
    pub fn id(&mut self, id: impl Into<String>) -> &mut Self {
        self.id = id.into();
        self
    }

    /// (optional) The password of the paste.
    pub fn password(&mut self, password: impl Into<String>) -> &mut Self {
        self.password = Some(password.into());
        self
    }
}
/// The builder to create a paste.
#[derive(Debug, Default)]
pub struct PasteBuilder {
    pub filename: String,
    pub content: String,
    pub expires: Option<Expiry>,
    pub password: Option<String>,
}

impl PasteBuilder {
    /// The filename for the paste.
    pub fn filename(&mut self, filename: impl Into<String>) -> &mut Self {
        self.filename = filename.into();
        self
    }

    /// The content for the paste.
    pub fn content(&mut self, content: impl Into<String>) -> &mut Self {
        self.content = content.into();
        self
    }

    /// (optional) The expiration date for the paste.
    pub fn expires(&mut self, expires: Expiry) -> &mut Self {
        self.expires = Some(expires);
        self
    }

    /// (optional) The password for the paste.
    pub fn password(&mut self, password: impl Into<String>) -> &mut Self {
        self.password = Some(password.into());
        self
    }
}

/// The builder to create multiple pastes.
#[derive(Debug, Default)]
pub struct PastesBuilder {
    pub files: Vec<PasteBuilder>,
}

impl PastesBuilder {
    pub fn file(
        &mut self,
        paste: impl FnOnce(&mut PasteBuilder) -> &mut PasteBuilder,
    ) -> &mut Self {
        let mut builder = PasteBuilder::default();
        let data = paste(&mut builder);
        self.files.push(take(data));
        self
    }
}

/// The builder to build options for getting user pastes.
#[derive(Debug)]
pub struct UserPastesOptions {
    pub limit: i32,
    pub page: i32,
}

impl UserPastesOptions {
    /// The limit of pastes to be shown. Defaults to 50.
    pub fn limit(&mut self, limit: i32) -> &mut Self {
        self.limit = limit;
        self
    }

    /// The page number to be shown. Defaults to 1.
    pub fn page(&mut self, page: i32) -> &mut Self {
        self.page = page;
        self
    }
}

impl Default for UserPastesOptions {
    fn default() -> Self {
        UserPastesOptions { limit: 50, page: 1 }
    }
}
