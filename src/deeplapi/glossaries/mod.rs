pub mod api;

use super::LangCode;
use super::DpTran;
use super::DeeplAPIError;
use super::connection;
use super::ApiKeyType;

use serde::{Deserialize, Serialize};

/// Glossary ID
pub type GlossaryID = String;

/// Glossaries dictionary struct
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GlossaryDictionary {
    pub source_lang: LangCode,
    pub target_lang: LangCode,
    pub entries: Vec<(String, String)>,
    pub entries_format: api::GlossariesApiFormat,
    pub entry_count: usize,
}

/// Glossary struct
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Glossary {
    pub name: String,
    pub id: Option<GlossaryID>,
    pub dictionaries: Vec<GlossaryDictionary>,
}

/// For the glossary API.  
/// Get a list of registered glossaries.
pub fn get_registered_glossaries(api: &DpTran) -> Result<Vec<Glossary>, DeeplAPIError> {
    let glossaries_list = api::GlossariesApiList::get_registered_dictionaries(api).map_err(|e| DeeplAPIError::GlossaryError(e.to_string()))?;
    let mut result: Vec<Glossary> = Vec::new();
    for glossary_data in glossaries_list.glossaries.iter() {
        // Convert GlossariesApiResponseData to Glossary
        let dictionaries: Vec<GlossaryDictionary> = glossary_data.dictionaries.iter().map(|dict_data| {
            GlossaryDictionary {
                source_lang: dict_data.source_lang.clone(),
                target_lang: dict_data.target_lang.clone(),
                entries: Vec::new(),  // Entries are not included in the list API response
                entries_format: api::GlossariesApiFormat::Tsv,  // Default to Tsv
                entry_count: dict_data.entry_count as usize,
            }
        }).collect();

        let glossary = Glossary {
            name: glossary_data.name.clone(),
            dictionaries,
            id: Some(glossary_data.glossary_id.clone()),
        };
        result.push(glossary);
    }

    Ok(result)
}

/// For the glossary API.  
/// Get supported languages for Glossaries API.
pub fn get_glossary_supported_languages(api: &DpTran) -> Result<api::GlossariesApiSupportedLanguages, DeeplAPIError> {
    api::GlossariesApiSupportedLanguages::get(api).map_err(|e| DeeplAPIError::GlossaryError(e.to_string()))
}

/// For the glossary API.
/// Delete a glossary.
pub fn delete_glossary(api: &DpTran, glossary: &Glossary) -> Result<(), DeeplAPIError> {
    if let Some(glossary_id) = &glossary.id {
        api::delete_glossary(api, glossary_id).map_err(|e| DeeplAPIError::GlossaryError(e.to_string()))
    } else {
        Err(DeeplAPIError::GlossaryIsNotRegisteredError)
    }
}

impl GlossaryDictionary {
    /// Make a new GlossaryDictionary instance.
    pub fn new(source_lang: String, target_lang: String, entries: Vec<(String, String)>, entries_format: api::GlossariesApiFormat) -> Self {
        let entry_count = entries.len();
        GlossaryDictionary {
            source_lang,
            target_lang,
            entries,
            entries_format,
            entry_count: entry_count,
        }
    }

    /// Retrive glossary entries from DeepL API.
    /// 
    /// `api`: DpTran instance
    /// `glossary_id`: GlossaryID
    pub fn retrieve_entries(&mut self, api: &DpTran, glossary_id: &GlossaryID) -> Result<(), DeeplAPIError> {
        let mut dictionary = api::GlossariesApiDictionaryPostData::new(
            &self.source_lang,
            &self.target_lang,
            &String::new(),  // entries will be retrieved
            &self.entries_format.to_string(),
        );
        let dictionary = dictionary.retrieve_entries(api, glossary_id).map_err(|e| DeeplAPIError::GlossaryError(e.to_string()))?;
        self.entries = Vec::new();
        for (source, target) in dictionary.get_entries_iter() {
            self.entries.push((source, target));
        }
        self.entry_count = self.entries.len();
        Ok(())
    }
}

impl Glossary {
    /// Make a new Glossary instance.
    /// 
    /// `name`: Glossary name
    /// `dictionaries`: Vec<GlossaryDictionary>
    pub fn new(name: String, dictionaries: Vec<GlossaryDictionary>) -> Self {
        Glossary {
            name,
            dictionaries,
            id: None,
        }
    }

    /// Retrive glossary details from DeepL API without entries.
    ///
    /// `api`: DpTran instance
    /// `id`: GlossaryID
    pub fn retrieve_details(&mut self, api: &DpTran, id: &GlossaryID) -> Result<(), DeeplAPIError> {
        let glossary_data = api::GlossariesApiResponseData::get_glossary_details(api, id).map_err(|e| DeeplAPIError::GlossaryError(e.to_string()))?;
        self.name = glossary_data.name;
        self.dictionaries = glossary_data.dictionaries.iter().map(|dict_data| {
            GlossaryDictionary {
                source_lang: dict_data.source_lang.clone(),
                target_lang: dict_data.target_lang.clone(),
                entries: Vec::new(),  // Entries are not included in the details API response
                entries_format: api::GlossariesApiFormat::Tsv,  // Default to Tsv
                entry_count: dict_data.entry_count as usize,
            }
        }).collect();
        Ok(())
    }

    /// Send glossary to DeepL API and create a glossary.  
    /// 
    /// `api`: DpTran instance
    pub fn send(&mut self, api: &DpTran) -> Result<GlossaryID, DeeplAPIError> {
        // Make Vec<GlossariesApiDictionaryPostData>
        let dictionaries: Vec<api::GlossariesApiDictionaryPostData> = self.dictionaries.iter().map(|dict| {
            // Prepare entries
            let entries = match dict.entries_format {
                api::GlossariesApiFormat::Tsv => {
                    dict.entries.iter().map(|(source, target)| format!("{}\t{}", source, target)).collect::<Vec<String>>().join("\n")
                },
                api::GlossariesApiFormat::Csv => {
                    dict.entries.iter().map(|(source, target)| format!("\"{}\",\"{}\"", source.replace("\"", "\"\""), target.replace("\"", "\"\""))).collect::<Vec<String>>().join("\n")
                },
            };

            api::GlossariesApiDictionaryPostData::new(
                &dict.source_lang,
                &dict.target_lang,
                &entries,
                &dict.entries_format.to_string(),
            )
        }).collect();

        // Make a new GlossariesApiPostData instance
        let glossary = api::GlossariesApiPostData::new(
            self.name.clone(),
            dictionaries,
        );

        // Send glossary to DeepL API
        let res = glossary.send(api).map_err(|e| DeeplAPIError::GlossaryError(e.to_string()))?;

        // Set and return glossary ID
        self.id = Some(res.glossary_id);

        Ok(self.id.as_ref().unwrap().clone())
    }

    /// Edit glossary content and update it on DeepL API.
    /// 
    /// `api`: DpTran instance
    pub fn update(&self, api: &DpTran) -> Result<(), DeeplAPIError> {
        if let Some(glossary_id) = &self.id {
            // Make Vec<GlossariesApiDictionaryPostData>
            let dictionaries: Vec<api::GlossariesApiDictionaryPostData> = self.dictionaries.iter().map(|dict| {
                // Prepare entries
                let entries = match dict.entries_format {
                    api::GlossariesApiFormat::Tsv => {
                        dict.entries.iter().map(|(source, target)| format!("{}\t{}", source, target)).collect::<Vec<String>>().join("\n")
                    },
                    api::GlossariesApiFormat::Csv => {
                        dict.entries.iter().map(|(source, target)| format!("\"{}\",\"{}\"", source.replace("\"", "\"\""), target.replace("\"", "\"\""))).collect::<Vec<String>>().join("\n")
                    },
                };

                api::GlossariesApiDictionaryPostData::new(
                    &dict.source_lang,
                    &dict.target_lang,
                    &entries,
                    &dict.entries_format.to_string(),
                )
            }).collect();

            // Make a new GlossariesApiPostData instance
            let glossary = api::GlossariesApiPostData::new(
                self.name.clone(),
                dictionaries,
            );

            // Update glossary on DeepL API
            api::patch_glossary(api, glossary_id, &glossary).map_err(|e| DeeplAPIError::GlossaryError(e.to_string()))
        } else {
            Err(DeeplAPIError::GlossaryIsNotRegisteredError)
        }
    }
}

/// To run these tests, you need to set the environment variable `DPTRAN_DEEPL_API_KEY` to your DeepL API key.  
/// You should run these tests with ``cargo test -- --test-threads=1``
/// because the DeepL API has a limit on the number of requests per second.
/// And also, you need to run the dummy server for the DeepL API to test the API endpoints.
///   $ pip3 install -r requirements.txt
///   $ uvicorn dummy_api_server.main:app --reload
#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn impl_glossary_dictionary_new() {
        let entries = vec![
            ("Hello".to_string(), "こんにちは".to_string()),
            ("World".to_string(), "世界".to_string()),
        ];
        let dict = GlossaryDictionary::new("EN".to_string(), "JA".to_string(), entries.clone(), api::GlossariesApiFormat::Tsv);
        assert_eq!(dict.source_lang, "EN");
        assert_eq!(dict.target_lang, "JA");
        assert_eq!(dict.entries, entries);
        assert_eq!(dict.entries_format, api::GlossariesApiFormat::Tsv);
        assert_eq!(dict.entry_count, 2);
    }

    #[test]
    fn impl_glossary_new() {
        let entries = vec![
            ("Hello".to_string(), "こんにちは".to_string()),
            ("World".to_string(), "世界".to_string()),
        ];
        let dict = GlossaryDictionary::new("EN".to_string(), "JA".to_string(), entries.clone(), api::GlossariesApiFormat::Tsv);
        let glossary = Glossary::new("Test Glossary".to_string(), vec![dict.clone()]);
        assert_eq!(glossary.name, "Test Glossary");
        assert_eq!(glossary.dictionaries.len(), 1);
        assert_eq!(glossary.dictionaries[0].source_lang, "EN");
        assert_eq!(glossary.dictionaries[0].target_lang, "JA");
        assert_eq!(glossary.dictionaries[0].entries, entries);
        assert_eq!(glossary.dictionaries[0].entries_format, api::GlossariesApiFormat::Tsv);
        assert_eq!(glossary.dictionaries[0].entry_count, 2);
        assert_eq!(glossary.id, None);
    }
}
