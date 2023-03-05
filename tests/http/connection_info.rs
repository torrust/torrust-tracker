use torrust_tracker::tracker::auth::Key;

#[derive(Clone, Debug)]
pub struct ConnectionInfo {
    pub bind_address: String,
    pub key_id: Option<Key>,
}

impl ConnectionInfo {
    pub fn anonymous(bind_address: &str) -> Self {
        Self {
            bind_address: bind_address.to_string(),
            key_id: None,
        }
    }
}
