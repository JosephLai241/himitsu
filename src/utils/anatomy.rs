//! Contains utilities for setting metadata for the stored secret. In other words, the anatomy of
//! the secret.

use chrono::Local;

use crate::models::metadata::Anatomy;

/// Create a new `Anatomy` for a particular secret.
pub fn create_new_anatomy(category: String, label: String, tags: Vec<String>) -> Anatomy {
    Anatomy {
        category,
        date_created: Local::now().format("%m-%d-%Y %H:%M:%S").to_string(),
        label,
        last_accessed: None,
        tags,
    }
}

/// Create a new `Anatomy` from an existing `Anatomy`.
pub fn create_anatomy_from_existing(
    category: &Option<String>,
    existing_anatomy: &Anatomy,
    tags: &Option<Vec<String>>,
) -> Anatomy {
    let category = match category {
        Some(category_name) => category_name.to_string(),
        None => "None".to_string(),
    };
    let tags = match tags {
        Some(tag_values) => tag_values.clone(),
        None => vec![],
    };

    Anatomy {
        category,
        date_created: existing_anatomy.date_created.clone(),
        last_accessed: Some(Local::now().format("%m-%d-%Y %H:%M:%S").to_string()),
        tags,
    }
}
