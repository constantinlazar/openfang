/// Email (IMAP/SMTP) channel adapter configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct EmailConfig {
    /// IMAP server host.
    pub imap_host: String,
    /// IMAP port (993 for TLS).
    pub imap_port: u16,
    /// SMTP server host.
    pub smtp_host: String,
    /// SMTP port (587 for STARTTLS).
    pub smtp_port: u16,
    /// Email address (used for both IMAP and SMTP).
    pub username: String,
    /// Env var name holding the password (basic auth).
    pub password_env: String,
    /// Authentication method: "basic" or "oauth2" (default: "basic").
    /// When "oauth2", use oauth2_* fields instead of password_env.
    #[serde(default = "default_basic_auth_type")]
    pub auth_type: String,
    /// Env var name holding the OAuth2 access token.
    #[serde(default)]
    pub access_token_env: Option<String>,
    /// Env var name holding the OAuth2 refresh token.
    #[serde(default)]
    pub refresh_token_env: Option<String>,
    /// OAuth2 token endpoint URL (e.g., for Office 365: https://login.microsoftonline.com/common/oauth2/v2.0/token).
    #[serde(default)]
    pub token_endpoint: Option<String>,
    /// Env var name holding the OAuth2 client ID.
    #[serde(default)]
    pub client_id_env: Option<String>,
    /// Env var name holding the OAuth2 client secret.
    #[serde(default)]
    pub client_secret_env: Option<String>,
    /// Poll interval in seconds.
    pub poll_interval_secs: u64,
    /// IMAP folders to monitor.
    #[serde(default, deserialize_with = "deserialize_string_or_int_vec")]
    pub folders: Vec<String>,
    /// Only process emails from these senders (empty = all).
    #[serde(default, deserialize_with = "deserialize_string_or_int_vec")]
    pub allowed_senders: Vec<String>,
    /// Default agent name to route messages to.
    pub default_agent: Option<String>,
    /// Per-channel behavior overrides.
    #[serde(default)]
    pub overrides: ChannelOverrides,
}

fn default_basic_auth_type() -> String {
    "basic".to_string()
}

impl Default for EmailConfig {
    fn default() -> Self {
        Self {
            imap_host: String::new(),
            imap_port: 993,
            smtp_host: String::new(),
            smtp_port: 587,
            username: String::new(),
            password_env: "EMAIL_PASSWORD".to_string(),
            auth_type: "basic".to_string(),
            access_token_env: None,
            refresh_token_env: None,
            token_endpoint: None,
            client_id_env: None,
            client_secret_env: None,
            poll_interval_secs: 30,
            folders: vec!["INBOX".to_string()],
            allowed_senders: vec![],
            default_agent: None,
            overrides: ChannelOverrides::default(),
        }
    }
}