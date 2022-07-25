//! Contains structs used for `skeletons` metadata.

use std::collections::HashMap;

use chrono::Local;
use serde::{Deserialize, Serialize};

/// Contains metadata for each secret.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Anatomy {
    /// The category this secret belongs in.
    pub category: String,
    /// The date this secret was created.
    pub date_created: String,
    /// The label associated with this secret.
    pub label: String,
    /// The date this secret was last accessed.
    pub last_accessed: Option<String>,
    /// The tags associated with this secret.
    pub tags: Vec<String>,
}

impl Anatomy {
    /// Create a new `Anatomy` for a secret from a defined `category`, `label`, and `tags`.
    pub fn create_from(category: String, label: String, tags: Vec<String>) -> Self {
        Self {
            category,
            date_created: Local::now().format("%m-%d-%Y %H:%M:%S").to_string(),
            label,
            last_accessed: None,
            tags,
        }
    }
}

/// Contains the lookup table for secrets.
///
/// An example of what this looks like:
///
/// ```
/// LookupTable {
///     table: {
///         "5d5c6f5b57bd22dded4046fa2eae5a64ec5aa5e3d6ba7420084a6245b7284ac4": { // Some hash
///             category: "Unclassified",
///             date_created: "2006-06-06 06:06:06",
///             label: "Some secret",
///             last_accessed: None,
///             tags: ["bottom-secret"]
///         },
///         "08aa7ed52b41ec21fa43676e89b96517a59c74fb0a79160468cf35102f02d281": { // Some hash
///             ...
///         },
///         ...
///     }
/// }
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct LookupTable {
    /// The values within the lookup table.
    pub table: HashMap<String, Anatomy>,
}

impl LookupTable {
    /// Create a new `LookupTable`.
    pub fn new() -> LookupTable {
        LookupTable {
            table: HashMap::new(),
        }
    }
}

/// This struct temporarily holds a modified entry in the lookup table while searching for matches,
/// which includes the secret's hash ID as well as its `Anatomy`.
#[derive(Debug, Deserialize, Serialize)]
pub struct LookupMatch {
    /// The secret's corresponding `Anatomy`.
    pub anatomy: Anatomy,
    /// The secret's hash ID.
    pub hash: String,
}

impl LookupMatch {
    /// Create a `LookupMatch` from the secret's hash and `Anatomy`.
    pub fn create(anatomy: Anatomy, hash: String) -> Self {
        Self { anatomy, hash }
    }
}
