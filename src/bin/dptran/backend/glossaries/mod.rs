use std::fmt;

// Stored glossaries (Vec)
pub struct GlossariesWrapper {
    glossaries: Vec<dptran::glossaries::Glossary>
}

#[derive(Debug, PartialEq)]
pub enum GlossariesError {
    FailToReadCache(String),
    WordPairMustBeEven,
}
impl fmt::Display for GlossariesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GlossariesError::FailToReadCache(ref e) => write!(f, "Failed to read glossaries data: {}", e),
            GlossariesError::WordPairMustBeEven => write!(f, "The number of word pairs to add must be even."),
        }
    }
}

impl GlossariesWrapper {
    /// Get registered glossaries in the DeepL API.
    /// Returns a reference to the vector of stored glossaries.
    pub fn get_glossaries(dptran: &dptran::DpTran) -> Result<Self, GlossariesError> {
        let ret = dptran::glossaries::get_registered_glossaries(dptran).map_err(|e| GlossariesError::FailToReadCache(e.to_string()))?;
        Ok(Self {
            glossaries: ret,
        })
    }

    /// Get registered glossaries data by name.
    /// Returns a reference to the glossary if found.
    pub fn search_by_name(&self, name: &str) -> Result<Option<dptran::glossaries::Glossary>, GlossariesError> {
        for glossary in self.glossaries.iter() {
            if glossary.name == name {
                return Ok(Some(glossary.clone()));
            }
        }
        Ok(None)
    }

    /// Get registered glossaries data by glossary ID.
    /// Returns a reference to the glossary if found.
    pub fn search_by_id(&self, glossary_id: &dptran::glossaries::GlossaryID) -> Result<Option<dptran::glossaries::Glossary>, GlossariesError> {
        for glossary in self.glossaries.iter() {
            if let Some(id) = &glossary.id {
                if id == glossary_id {
                    return Ok(Some(glossary.clone()));
                }
            }
        }
        Ok(None)
    }
    
    /// Add a new glossary to the stored glossaries.
    /// Returns nothing.
    /// # Arguments
    /// * `glossary` - The glossary to be added.
    pub fn add_glossary(&mut self, dptran: &dptran::DpTran, glossary: dptran::glossaries::Glossary) -> Result<dptran::glossaries::GlossaryID, GlossariesError> {
        // If there is a glossary with the same name, replace it.
        self.glossaries.retain(|g| g.name != glossary.name);
        // Send glossary to DeepL API
        let mut glossary = glossary.clone();
        let glossary_id = glossary.send(dptran).map_err(|e| GlossariesError::FailToReadCache(e.to_string()))?;
        self.glossaries.push(glossary);
        Ok(glossary_id)
    }

    /// Remove a glossary by name.
    /// Returns nothing.
    /// # Arguments
    /// * `name` - The name of the glossary to be removed.
    pub fn remove_glossary_by_name(&mut self, dptran: &dptran::DpTran, glossary: &dptran::glossaries::Glossary) -> Result<(), GlossariesError> {
        dptran::glossaries::delete_glossary(dptran, glossary).map_err(|e| GlossariesError::FailToReadCache(e.to_string()))?;
        self.glossaries.retain(|g| g.name != glossary.name);
        Ok(())
    }

    /// Get all glossaries.
    pub fn get_all_glossaries(&self) -> &Vec<dptran::glossaries::Glossary> {
        &self.glossaries
    }
}

/// Get supported languages for Glossaries API.
pub fn get_glossary_supported_languages(api: &dptran::DpTran) -> Result<dptran::glossaries::api::GlossariesApiSupportedLanguages, GlossariesError> {
    dptran::glossaries::get_glossary_supported_languages(api).map_err(|e| GlossariesError::FailToReadCache(e.to_string()))
}

/// Vec<String> to Vec<(String, String)>
pub fn vec_string_to_word_pairs(vec: &Vec<String>) -> Result<Vec<(String, String)>, GlossariesError> {
    if vec.len() % 2 != 0 {
        return Err(GlossariesError::WordPairMustBeEven);
    }
    let mut word_pairs: Vec<(String, String)> = Vec::new();
    for i in (0..vec.len()).step_by(2) {
        word_pairs.push((vec[i].clone(), vec[i+1].clone()));
    }
    Ok(word_pairs)
}

//#[cfg(test)]
//include!("./tests.rs");
