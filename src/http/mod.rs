pub use self::errors::*;
pub use self::filters::*;
pub use self::handlers::*;
pub use self::request::*;
pub use self::response::*;
pub use self::routes::*;
pub use self::server::*;

pub mod server;
pub mod request;
pub mod response;
pub mod errors;
pub mod routes;
pub mod handlers;
pub mod filters;

pub type Bytes = u64;
pub type WebResult<T> = std::result::Result<T, warp::Rejection>;
