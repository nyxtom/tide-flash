use serde::{Deserialize, Serialize};

pub mod cookies;
mod flash_store;
mod middleware;
mod request_ext;
mod response_ext;

pub use flash_store::FlashStore;
pub use middleware::FlashMiddleware;

pub mod prelude {
    pub use crate::request_ext::*;
    pub use crate::response_ext::*;
}

#[derive(Default, Clone)]
pub struct Flash {
    pub messages: Vec<FlashMessage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlashMessage {
    pub level: String,
    pub message: String,
}
