use ritmo_domain::{Format, Genre, Role};

pub trait I18nDisplayable {
    fn display_name(&self, locale: &str) -> String;
}

impl I18nDisplayable for Format {
    fn display_name(&self, locale: &str) -> String {
        rust_i18n::t!(self.i18n_key.as_str(), locale = locale).to_string()
    }
}

impl I18nDisplayable for Genre {
    fn display_name(&self, locale: &str) -> String {
        rust_i18n::t!(self.i18n_key.as_str(), locale = locale).to_string()
    }
}

impl I18nDisplayable for Role {
    fn display_name(&self, locale: &str) -> String {
        rust_i18n::t!(self.i18n_key.as_str(), locale = locale).to_string()
    }
}
