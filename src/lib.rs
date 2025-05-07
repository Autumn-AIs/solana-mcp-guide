pub mod tests;

use serde::{Deserialize, Serialize};
use url::Url;

// Define a local version of ReadResourceRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadResourceRequest {
    pub uri: Url,
}