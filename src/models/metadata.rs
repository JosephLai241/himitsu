//! Contains structs used for `skeletons` metadata.

use serde::{Deserialize, Serialize};

/// Contains the metadata for each secret's `anatomy`.
#[derive(Debug, Deserialize, Serialize)]
pub struct Anatomy {
    /// The category this secret belongs in.
    pub category: String,
    /// The date this secret was created.
    pub date_created: String,
    /// The date this secret was last accessed.
    pub last_accessed: Option<String>,
    /// The tags associated with this secret.
    pub tags: Vec<String>,
}
