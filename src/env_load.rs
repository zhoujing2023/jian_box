use crate::my_error::MyError;
use std::error::Error;
use std::path::PathBuf;
use std::sync::OnceLock;

pub struct EnvLad;

/// home 目录
pub static HOME_DIR: OnceLock<PathBuf> = OnceLock::new();

/// 项目名称
pub static PROJECT_NAME: OnceLock<String> = OnceLock::new();

impl EnvLad {
    pub fn load() -> Result<(), Box<dyn Error>> {
        // 获取 home 目录
        if HOME_DIR.get().is_none() {
            let home_dir = match dirs::home_dir() {
                Some(home_dir) => home_dir,
                None => {
                    eprintln!("获取 home 目录失败");
                    return Err(Box::new(MyError::new("获取 home 目录失败".to_string())));
                }
            };
            HOME_DIR.set(home_dir).unwrap_or_else(|err| {
                eprintln!("赋值 home 目录失败：{}", err.display());
            });
        }

        // 获取项目名称
        if PROJECT_NAME.get().is_none() {
            let project_name = env!("CARGO_PKG_NAME");
            PROJECT_NAME
                .set(project_name.to_string())
                .unwrap_or_else(|err| {
                    eprintln!("赋值项目名称失败：{}", err);
                })
        }
        println!("获取环境信息成功~");
        println!(
            "home目录：{:?}\n项目名称：{:?}",
            HOME_DIR.get(),
            PROJECT_NAME.get()
        );
        Ok(())
    }
}
