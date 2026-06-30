use crate::config::Config;
use crate::model::{AppUsageData, Apps};
use anyhow::{Context, Result, anyhow};
use std::collections::HashMap;
use std::fs;
use std::fs::read_to_string;

pub struct AppUsage;

impl AppUsage {
    /// `load` 加载 usage 文件
    ///
    /// # Examples
    /// ```
    /// use jian_box::{AppUsage, Config};
    /// let config = Config::load().unwrap();
    /// let usage = AppUsage::load(&config);
    /// ```
    pub fn load(config: &Config) -> Option<AppUsageData> {
        let content = match Self::read_usage_file(&config) {
            Ok(content) => content,
            Err(error) => {
                eprintln!("读取配置文件出现异常：{}", error);
                return None;
            }
        };
        if content.is_empty() {
            return None;
        }
        let data: AppUsageData = match serde_json::from_str(&content) {
            Ok(content) => content,
            Err(err) => {
                eprintln!("解析配置文件失败，错误信息：{}", err);
                return None;
            }
        };
        Some(data)
    }

    ///  `read_config_file` 读取 "usage.json" 配置文件
    /// 如果文件不存在将自动新建
    fn read_usage_file(config: &Config) -> Result<String> {
        let config_file = config.usage_file_path();
        match fs::exists(&config_file) {
            Ok(exists) => {
                if !exists {
                    let file_dir = &config.usage_path();
                    match fs::exists(&file_dir) {
                        Ok(exists) => {
                            // 创建目录
                            if !exists {
                                fs::create_dir(&file_dir).with_context(|| {
                                    format!("创建 {} 目录失败", file_dir.display())
                                })?;
                            }
                            // 创建配置文件
                            fs::File::create(&config_file).with_context(|| {
                                format!("创建 {} 文件失败", config_file.display())
                            })?;
                        }
                        Err(e) => {
                            return Err(anyhow!(
                                "判断 {} 目录是否存在出现异常：{}",
                                file_dir.display(),
                                e
                            ));
                        }
                    }
                    return Ok(String::default());
                }
            }
            Err(e) => {
                return Err(anyhow!(
                    "判断 {} 目录是否存在出现异常：{}",
                    config_file.display(),
                    e
                ));
            }
        }
        let content = read_to_string(&config_file)
            .with_context(|| format!("读取配置文件 {} 失败", config_file.display()))?;
        Ok(content)
    }

    /// `update_usage_file` 更新 "usage.json" 文件
    ///
    /// # Examples
    /// ```
    /// use jian_box::{AppLoader, AppUsage, Config};
    /// let config = Config::load().unwrap();
    /// let apps = AppLoader::load(&config);
    /// AppUsage::update_usage_file(&apps,&config);
    /// ```
    pub fn update_usage_file(apps: &Apps, config: &Config) {
        let mut usage_data = HashMap::<String, u32>::new();
        for app in apps {
            usage_data.insert(app.desktop_file.clone(), *app.score.borrow());
        }
        let app_usage_data = AppUsageData { scores: usage_data };
        let json_str = match serde_json::to_string_pretty(&app_usage_data) {
            Ok(json_str) => json_str,
            Err(err) => {
                eprintln!("解析为json字符串失败：{}", err);
                return;
            }
        };
        let config_file = &config.usage_file_path();
        if let Err(err) = fs::write(config_file, json_str) {
            eprintln!("更新 usage 失败：{}", err);
        }
    }
}
