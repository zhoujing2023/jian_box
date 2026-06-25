use crate::model::{AppEntry, Apps};

pub struct SearchEngine {
    apps: Apps,
}

impl SearchEngine {
    /// `new` 创建实例
    pub fn new(apps: Apps) -> Self {
        Self { apps }
    }

    ///  `search` 搜索
    ///
    /// # Examples
    ///
    /// ```
    ///   let results = search(&apps, String::from("wechat"));
    ///   if let Some(results) = results {
    ///       println!("{:#?}", results);
    ///       println!("查询到的数量：{}", results.len());
    ///   }
    /// ```
    pub fn search(&self, keyword: &str) -> Vec<&AppEntry> {
        let keyword_lower = keyword.to_lowercase();
        let keyword_lower = keyword_lower.as_str();
        self.apps
            .iter()
            .filter(|app| Self::fuzzy_match(&app.search_key, keyword_lower))
            .collect()
    }

    /// `fuzzy_match` 模糊匹配
    ///
    /// # Examples
    /// ```
    /// let name = "WeChat".to_lowercase();
    /// println!("{}", fuzzy_match(&name, "we"));
    /// println!("{}", fuzzy_match(&name, "tw"));
    /// ```
    fn fuzzy_match(text: &str, pattern: &str) -> bool {
        if pattern.is_empty() {
            return false;
        }

        let mut pattern_chars = pattern.chars();
        let mut cur_pattern_char = pattern_chars.next();

        for char in text.chars() {
            if let Some(p_char) = cur_pattern_char {
                if char == p_char {
                    cur_pattern_char = pattern_chars.next();
                }
            } else {
                return true;
            }
        }
        cur_pattern_char.is_none()
    }
}
