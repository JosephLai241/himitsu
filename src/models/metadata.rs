//! Contains structs used for `skeletons` metadata.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Contains the metadata for each secret's `anatomy`.
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
    /// Create a new LookupTable.
    pub fn new() -> LookupTable {
        LookupTable {
            table: HashMap::new(),
        }
    }
}
