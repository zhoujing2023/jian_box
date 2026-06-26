use crate::env_load::{HOME_DIR, PROJECT_NAME};
use crate::model::{AppUsageData, Apps};
use crate::my_error::MyError;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::fs::read_to_string;

pub struct AppUsage;

impl AppUsage {
    pub fn load() -> Option<AppUsageData> {
        let content = match Self::read_usage_file() {
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

    ///  `read_config_file` 读取配置文件
    fn read_usage_file() -> Result<String, Box<dyn Error>> {
        let Some(home_dir) = HOME_DIR.get() else {
            return Err(Box::new(MyError::new("获取 home 目录失败".to_string())));
        };
        let Some(project_name) = PROJECT_NAME.get() else {
            return Err(Box::new(MyError::new("获取项目名称失败".to_string())));
        };

        let config_file = home_dir.join(format!(".config/{}/usage.json", project_name));
        match fs::exists(&config_file) {
            Ok(exists) => {
                if !exists {
                    let file_dir = home_dir.join(format!(".config/{}", project_name));
                    match fs::exists(&file_dir) {
                        Ok(exists) => {
                            // 创建目录
                            if !exists {
                                match fs::create_dir(&file_dir) {
                                    Ok(_) => {}
                                    Err(e) => {
                                        eprintln!(
                                            "创建 {} 目录出现异常：{}",
                                            file_dir.display(),
                                            e
                                        );
                                        return Err(Box::new(e));
                                    }
                                }
                            }
                            // 创建配置文件
                            let filename =
                                home_dir.join(format!(".config/{}/usage.json", project_name));
                            match fs::File::create(&filename) {
                                Ok(_) => (),
                                Err(e) => {
                                    eprintln!("创建 {} 文件出现异常：{}", filename.display(), e);
                                    return Err(Box::new(e));
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("判断 {} 目录是否存在出现异常：{}", file_dir.display(), e);
                            return Err(Box::new(e));
                        }
                    }
                    return Ok(String::default());
                }
            }
            Err(e) => {
                eprintln!("判断 {} 目录是否存在出现异常：{}", config_file.display(), e);
                return Err(Box::new(e));
            }
        }
        let content = match read_to_string(&config_file) {
            Ok(content) => content,
            Err(err) => {
                eprintln!("读取配置文件：{}，出现异常：{}", config_file.display(), err);
                return Err(Box::new(err));
            }
        };
        Ok(content)
    }

    /// `update_usage_file` 更新 usage 文件
    pub fn update_usage_file(apps: &Apps) {
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
        let Some(home_dir) = HOME_DIR.get() else {
            eprintln!("获取 home 目录失败");
            return;
        };
        let Some(project_name) = PROJECT_NAME.get() else {
            eprintln!("获取项目名称失败");
            return;
        };
        let config_file = home_dir.join(format!(".config/{}/usage.json", project_name));
        fs::write(config_file, json_str).unwrap_or_else(|e| {
            eprintln!("更新 usage 失败：{}", e);
            return;
        })
    }
}