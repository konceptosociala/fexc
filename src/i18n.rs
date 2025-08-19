use std::{collections::HashMap, ops::Index};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    #[serde(rename = "en")]
    English,
    #[serde(rename = "uk")]
    Ukrainian,
}

const EN: &str = include_str!("../assets/i18n/en.toml");
const UK: &str = include_str!("../assets/i18n/uk.toml");

pub struct I18n(HashMap<Language, toml::Table>);

impl Default for I18n {
    fn default() -> Self {
        I18n::new()
    }
}

impl I18n {
    pub fn new() -> Self {
        I18n(HashMap::from([
            (Language::English, toml::de::from_str(EN).unwrap()),
            (Language::Ukrainian, toml::de::from_str(UK).unwrap()),
        ]))
    }

    pub fn get(&self, lang: &Language, key: &str) -> Option<&str> {
        self.0.get(lang)
            .and_then(|table| table.get(key))
            .and_then(|value| value.as_str())
    }
}

impl Index<(&Language, &str)> for I18n {
    type Output = str;

    fn index(&self, (lang, key): (&Language, &str)) -> &Self::Output {
        self.get(lang, key)
            .unwrap_or_else(|| {
                panic!("Key '{}' not found in language '{:?}'", key, lang)
            })
    }
}