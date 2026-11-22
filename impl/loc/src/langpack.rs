use crate::error::LocResult;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;

/// A language pack loaded from keywords.json + grammar.json.
#[derive(Debug, Clone)]
pub struct LanguagePack {
    pub iso: String,
    pub name: String,
    pub script: String,
    pub direction: String,
    pub grammar_variant: String,
    pub version: String,
    /// KW slot -> list of natural-language keyword strings.
    pub keywords: HashMap<String, Vec<String>>,
    /// TY slot -> natural-language type names.
    pub types: HashMap<String, Vec<String>>,
    pub number_format: NumberFormat,
    pub grammar: GrammarJson,
}

/// Number format hints (used by lofmt, not lexing per LEX-064).
#[derive(Debug, Clone, Deserialize)]
pub struct NumberFormat {
    pub decimal_sep: String,
    pub thousand_sep: String,
}

/// grammar.json contents (SPEC-02 §9).
#[derive(Debug, Clone, Deserialize)]
pub struct GrammarJson {
    pub word_order: String,
    pub word_segmentation: String,
    pub requires_explicit_lingua_directive: bool,
    pub script: String,
    pub bidi: String,
    pub preferred_decimal_sep: Option<String>,
    pub indent_unit: Option<usize>,
    pub multiphrasal_keywords: Option<bool>,
}

impl LanguagePack {
    /// Load a language pack from langpacks/{iso}/.
    pub fn load(iso: &str, root: &Path) -> LocResult<Self> {
        let dir = root.join("langpacks").join(iso);

        let keywords_path = dir.join("keywords.json");
        let keywords_json: serde_json::Value =
            serde_json::from_slice(&std::fs::read(&keywords_path)
                .map_err(|e| crate::error::LocError::IoError(format!("cannot read {:?}: {}", keywords_path, e)))?)?;

        let iso = keywords_json["iso"].as_str().unwrap_or(iso).to_string();
        let name = keywords_json["name"].as_str().unwrap_or("").to_string();
        let script = keywords_json["script"].as_str().unwrap_or("Latin").to_string();
        let direction = keywords_json["direction"].as_str().unwrap_or("ltr").to_string();
        let grammar_variant = keywords_json["grammar_variant"].as_str().unwrap_or("svo").to_string();
        let version = keywords_json["version"].as_str().unwrap_or("").to_string();

        let mut keywords = HashMap::new();
        if let Some(kw) = keywords_json["keywords"].as_object() {
            for (slot, vals) in kw {
                if let Some(arr) = vals.as_array() {
                    let strings: Vec<String> = arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect();
                    keywords.insert(slot.clone(), strings);
                }
            }
        }

        let mut types = HashMap::new();
        if let Some(ty) = keywords_json["types"].as_object() {
            for (slot, vals) in ty {
                if let Some(arr) = vals.as_array() {
                    let strings: Vec<String> = arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect();
                    types.insert(slot.clone(), strings);
                }
            }
        }

        let number_format = NumberFormat {
            decimal_sep: keywords_json["number_format"]["decimal_sep"].as_str().unwrap_or(".").to_string(),
            thousand_sep: keywords_json["number_format"]["thousand_sep"].as_str().unwrap_or(",").to_string(),
        };

        let grammar = if let Ok(gjson) = std::fs::read(dir.join("grammar.json")) {
            serde_json::from_slice(&gjson).unwrap_or_default()
        } else {
            GrammarJson::default()
        };

        Ok(LanguagePack {
            iso,
            name,
            script,
            direction,
            grammar_variant,
            version,
            keywords,
            types,
            number_format,
            grammar,
        })
    }

    /// Get all keyword strings for a given KW slot.
    pub fn keyword_strings(&self, slot: &str) -> Vec<&str> {
        self.keywords.get(slot)
            .map(|v| v.iter().map(|s| s.as_str()).collect())
            .unwrap_or_default()
    }

    /// Reverse lookup: given a natural-language keyword string, return the KW slot.
    pub fn keyword_slot(&self, word: &str) -> Option<String> {
        for (slot, words) in &self.keywords {
            if words.iter().any(|w| w == word) {
                return Some(slot.clone());
            }
        }
        None
    }
}

impl Default for GrammarJson {
    fn default() -> Self {
        GrammarJson {
            word_order: "svo".into(),
            word_segmentation: "space".into(),
            requires_explicit_lingua_directive: false,
            script: "Latin".into(),
            bidi: "ltr".into(),
            preferred_decimal_sep: Some(".".into()),
            indent_unit: Some(4),
            multiphrasal_keywords: Some(false),
        }
    }
}

/// List available language packs under langpacks/.
pub fn list_packs(root: &Path) -> LocResult<Vec<String>> {
    let dir = root.join("langpacks");
    if !dir.exists() {
        return Ok(vec![]);
    }
    let mut packs = Vec::new();
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            packs.push(entry.file_name().to_string_lossy().to_string());
        }
    }
    packs.sort();
    Ok(packs)
}