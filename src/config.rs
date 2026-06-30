use anyhow::{Context, Result};
use std::env;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Config {
    pub home_dir: PathBuf,
    pub project_name: String,
    pub language: String,
}

impl Config {
    /// `load` 加载配置数据
    ///
    /// # Examples
    /// ```
    /// use jian_box::Config;
    /// let config = Config::load().unwrap();
    /// ```
    pub fn load() -> Result<Self> {
        let home_dir = dirs::home_dir().context("无法获取 Home 目录，请检查系统环境变量")?;
        let language = env::var("LANG")
            .or_else(|_| env::var("LC_MESSAGES"))
            .unwrap_or_else(|_| "en_US.UTF-8".to_string()) // 默认使用英文
            .split(".")
            .next()
            .map(|s| s.to_string())
            .unwrap_or_else(||"en_US".to_string());
        Ok(Self {
            home_dir,
            project_name: env!("CARGO_PKG_NAME").to_string(),
            language,
        })
    }

    /// `usage_file_path` 获取 usage 文件路径
    ///
    /// # Examples
    /// ```
    /// use std::path::{Path, PathBuf};
    /// use jian_box::Config;
    /// let config = Config::load().unwrap();
    /// let path = config.usage_file_path();
    /// assert!(path.ends_with(".config/jian_box/usage.json"));
    /// ```
    pub fn usage_file_path(&self) -> PathBuf {
        self.home_dir
            .join(format!(".config/{}/usage.json", self.project_name))
    }

    /// `usage_path` 获取 usage 路径
    ///
    /// # Examples
    /// ```
    /// use std::path::{Path, PathBuf};
    /// use jian_box::Config;
    /// let config = Config::load().unwrap();
    /// let path = config.usage_path();
    /// assert!(path.ends_with(".config/jian_box"));
    /// ```
    pub fn usage_path(&self) -> PathBuf {
        self.home_dir.join(format!(".config/{}", self.project_name))
    }
}
