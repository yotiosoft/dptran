use std::collections::HashMap;
use std::fmt;
use dptran::glossaries::api::GlossariesApiError;
use serde::{Deserialize, Serialize};
use confy;
use md5;

// An alias of dptran::glossaries::GlossaryDictionary
pub type StoredGlossaryDictionary = dptran::glossaries::GlossaryDictionary;

// An alias of dptran::Glossary
pub type StoredGlossary = dptran::glossaries::Glossary;

// Stored glossaries (Vec)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StoredGlossaries {
    pub saved_version: String,
    glossaries: Vec<StoredGlossary>
}
impl Default for StoredGlossaries {
    fn default() -> Self {
        Self {
            saved_version: env!("CARGO_PKG_VERSION").to_string(),
            glossaries: Vec::new()
        }
    }
}

// Stored glossaries wrapper
pub struct StoredGlossariesWrapper {
    glossaries_name: String,
    stored_glossaries: StoredGlossaries,
}

/// Cache error
#[derive(Debug, PartialEq)]
pub enum GlossariesError {
    FailToReadCache(String),
}
impl fmt::Display for GlossariesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GlossariesError::FailToReadCache(ref e) => write!(f, "Failed to read glossaries data: {}", e),
        }
    }
}

pub fn get_glossaries_data(glossaries_name: &str) -> Result<StoredGlossariesWrapper, GlossariesError> {
    let stored_glossaries = confy::load::<StoredGlossaries>("dptran", glossaries_name).map_err(|e| GlossariesError::FailToReadCache(e.to_string()))?;
    Ok(StoredGlossariesWrapper { 
        glossaries_name: glossaries_name.to_string().clone(),
        stored_glossaries 
    })
}

impl StoredGlossariesWrapper {
    fn save_glossaries_data(&self) -> Result<(), GlossariesError> {
        confy::store("dptran", self.glossaries_name.as_str(), self.stored_glossaries.clone()).map_err(|e| GlossariesError::FailToReadCache(e.to_string()))
    }

    /// Get stored glossaries
    /// Returns a reference to the vector of stored glossaries.
    pub fn get_glossaries(&self) -> &Vec<StoredGlossary> {
        &self.stored_glossaries.glossaries
    }

    /// Get stored glossary by name
    /// Returns an Option containing a reference to the glossary if found.
    pub fn get_glossary_by_name(&self, name: &str) -> Option<&StoredGlossary> {
        for glossary in &self.stored_glossaries.glossaries {
            if glossary.name == name {
                return Some(glossary);
            }
        }
        None
    }

    /// Add a new glossary to the stored glossaries.
    /// Returns nothing.
    /// # Arguments
    /// * `glossary` - The glossary to be added.
    pub fn add_glossary(&mut self, glossary: dptran::glossaries::Glossary) -> Result<(), GlossariesError> {
        // If there is a glossary with the same name, replace it.
        if let Some(existing_glossary) = self.stored_glossaries.glossaries.iter_mut().find(|g| g.name == glossary.name) {
            *existing_glossary = glossary;
        } else {
            self.stored_glossaries.glossaries.push(glossary);
        }
        self.save_glossaries_data()
    }

    /// Remove a glossary by name.
    /// Returns nothing.
    /// # Arguments
    /// * `name` - The name of the glossary to be removed.
    pub fn remove_glossary_by_name(&mut self, name: &str) -> Result<(), GlossariesError> {
        self.stored_glossaries.glossaries.retain(|g| g.name != name);
        self.save_glossaries_data()
    }
}

//#[cfg(test)]
//include!("./tests.rs");
