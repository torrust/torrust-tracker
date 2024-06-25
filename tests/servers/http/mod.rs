use torrust_tracker::servers::http::handle::Handle;
use torrust_tracker::servers::service;

pub mod asserts;
pub mod environment;
pub mod v1;

pub type Started<'a> = environment::Environment<service::Started<Handle>>;

//pub type Stopped<'a> = environment::Environment<service::Stopped>;
