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
        let mut results = Vec::with_capacity(10);
        let keyword_lower = keyword.to_lowercase();
        let keyword_lower = keyword_lower.as_str();
        for app in self.apps.iter() {
            let result = app.search_key.find(keyword_lower);
            if let Some(_) = result {
                results.push(app);
            }
        }
        results
    }
}
