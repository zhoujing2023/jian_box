pub type Apps = Vec<AppEntry>;

#[derive(Debug, Default)]
pub struct AppEntry {
    pub name: String,
    pub exec: String,
    pub desktop_file: String,
    pub search_key: String,
    pub comment: String,
    pub icon: Option<String>,
}
