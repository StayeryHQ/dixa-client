mod generated;

pub type DixaClient = generated::HttpClient;
pub use generated::client::ApiOpError as DixaError;
pub use generated::types::*;
