/// Represents the identity of a user within the system.
///
/// Fields:
/// - `id`: A unique identifier for the user.
/// - `username`: The user's chosen username.
/// - `email`: The user's email address.
/// - `verified`: Boolean flag indicating if the user's account is verified.
/// - `oauth_provider`: The name of the OAuth provider used for authentication.
/// - `oauth_id`: The user's identifier from the OAuth provider.
/// - `created_at`: Timestamp of when the account was created.
/// - `last_login`: Timestamp of the user's last login.
pub(crate) struct UserIdentity {
    id: usize,
    username: String,
    email: String,
    verified: bool,
    oauth_provider: String,
    oauth_id: String,
    created_at: String,
    last_login: String,
}

impl Default for UserIdentity {
    fn default() -> Self {
        Self {
            id: Default::default(),
            username: Default::default(),
            email: Default::default(),
            verified: Default::default(),
            oauth_provider: Default::default(),
            oauth_id: Default::default(),
            created_at: Default::default(),
            last_login: Default::default(),
        }
    }
}

impl UserIdentity {
    pub fn new() -> UserIdentityBuilder {
        let def = UserIdentity::default();
        UserIdentityBuilder {
            id: def.id,
            username: def.username,
            email: def.email,
            verified: def.verified,
            oauth_provider: def.oauth_provider,
            oauth_id: def.oauth_id,
            created_at: def.created_at,
            last_login: def.last_login,
        }
    }
}

/// Builder for creating a `UserIdentity` instance.
///
/// Provides a way to construct a `UserIdentity` with optional settings. Each field can be set independently.
///
/// Fields:
/// - `id`: A unique identifier for the user.
/// - `username`: The user's chosen username.
/// - `email`: The user's email address.
/// - `verified`: Boolean flag for the account's verification status.
/// - `oauth_provider`: The OAuth provider used for authentication.
/// - `oauth_id`: The user's identifier from the OAuth provider.
/// - `created_at`: Timestamp of account creation.
/// - `last_login`: Timestamp of the user's last login.
pub(crate) struct UserIdentityBuilder {
    id: usize,
    username: String,
    email: String,
    verified: bool,
    oauth_provider: String,
    oauth_id: String,
    created_at: String,
    last_login: String,
}

impl UserIdentityBuilder {
    pub fn username(&mut self, username: &str) -> &mut Self {
        self.username = String::with_capacity(username.len());
        self
    }
    
    pub fn email(&mut self, email: &str) -> &mut Self {
        self.email = String::with_capacity(email.len());
        self
    }

    pub fn verified(&mut self, verified: bool) -> &mut Self {
        self.verified = verified;
        self
    }

    pub fn oauth_provider(&mut self, oauth_provider: &str) -> &mut Self {
        self.username = String::with_capacity(oauth_provider.len());
        self
    }

    pub fn oauth_id(&mut self, oauth_id: &str) -> &mut Self {
        self.oauth_id = String::with_capacity(oauth_id.len());
        self
    }

    pub fn build(&mut self) -> UserIdentity {
        UserIdentity {
            id: 0,
            username: self.username.clone(),
            email: self.email.clone(),
            verified: self.verified,
            oauth_provider: self.oauth_provider.clone(),
            oauth_id: self.oauth_id.clone(),
            created_at: self.created_at.clone(),
            last_login: self.last_login.clone(),
        }
    }
}