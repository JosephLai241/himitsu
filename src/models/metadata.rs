//! Contains structs used for `himitsu` metadata.

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
///             category: "unclassified",
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

#[cfg(test)]
pub mod test_metadata {
    use super::*;

    /// Test whether a new `Anatomy` is created from the `create_from()` method.
    #[test]
    fn test_anatomy_create_from() {
        let category = "unclassified".to_string();
        let label = "some secret".to_string();
        let tags = vec!["some-tag".to_string()];

        let test_anatomy = Anatomy::create_from(category.clone(), label.clone(), tags.clone());

        assert!(test_anatomy.category == category);
        assert!(test_anatomy.label == label);
        assert!(test_anatomy.tags == tags);
    }

    /// Test whether a new `LookupTable` is created.
    #[test]
    fn test_lookuptable_new() {
        let test_lookup_table = LookupTable::new();

        assert!(test_lookup_table.table.is_empty());
    }

    /// Test whether a new `LookupMatch` is created.
    #[test]
    fn test_lookupmatch_create() {
        let anatomy = Anatomy {
            category: "unclassified".to_string(),
            date_created: "today".to_string(),
            label: "something".to_string(),
            last_accessed: None,
            tags: vec!["tag".to_string()],
        };
        let hash = "421c76d77563afa1914846b010bd164f395bd34c2102e5e99e0cb9cf173c1d87".to_string();

        let test_lookup_match = LookupMatch::create(anatomy.clone(), hash.clone());

        assert!(test_lookup_match.anatomy.category == anatomy.category);
        assert!(test_lookup_match.anatomy.date_created == anatomy.date_created);
        assert!(test_lookup_match.anatomy.label == anatomy.label);
        assert!(test_lookup_match.anatomy.last_accessed == anatomy.last_accessed);
        assert!(test_lookup_match.anatomy.tags == anatomy.tags);
        assert!(test_lookup_match.hash == hash);
    }
}
