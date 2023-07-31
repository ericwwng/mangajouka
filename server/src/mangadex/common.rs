#[derive(Eq, PartialEq, Hash)]
pub enum SupportedLanguage {
    English,
}

impl SupportedLanguage {
    pub fn get_supported_language_string(&self) -> &'static str {
        match self {
            SupportedLanguage::English => "en",
        }
    }
}
