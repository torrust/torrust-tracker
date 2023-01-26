use torrust_tracker::tracker::auth::KeyId;

#[derive(Clone, Debug)]
pub struct ConnectionInfo {
    pub bind_address: String,
    pub key_id: Option<KeyId>,
}

impl ConnectionInfo {
    pub fn anonymous(bind_address: &str) -> Self {
        Self {
            bind_address: bind_address.to_string(),
            key_id: None,
        }
    }
}
