use std::time::SystemTime;

lazy_static! {
    pub static ref TIME_AT_APP_START: SystemTime = SystemTime::now();
}
