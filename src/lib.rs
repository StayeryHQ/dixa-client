mod generated;

pub type DixaClient = generated::HttpClient;
pub use generated::client::HttpError as DixaError;
pub use generated::client::HttpResult as DixaResult;
pub use generated::types::*;
