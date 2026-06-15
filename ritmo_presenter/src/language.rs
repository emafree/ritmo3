use serde::Serialize;

#[derive(Debug, Clone)]
pub struct LanguageView {
    pub id: i64,
    pub name: String,
}

/// Role option for language-entity relationship (e.g. "original", "translation").
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct LangRoleItem {
    pub role_id: i64,
    pub role_key: String,
    pub role_label: String,
}

pub fn build_lang_role_items(roles: Vec<(i64, String, String)>) -> Vec<LangRoleItem> {
    roles
        .into_iter()
        .map(|(role_id, role_key, role_label)| LangRoleItem {
            role_id,
            role_key,
            role_label,
        })
        .collect()
}
