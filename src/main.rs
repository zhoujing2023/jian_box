mod app_loader;
mod app_runner;
mod app_usage;
mod env_load;
mod launcher;
mod model;
mod my_error;
mod search_engine;

use crate::app_loader::AppLoader;
use crate::env_load::EnvLad;
use crate::launcher::Launcher;
use crate::search_engine::SearchEngine;

fn main() {
    EnvLad::load().unwrap_or_else(|e| panic!("获取环境信息失败：{}", e));
    println!("********* 开始检索desktop *********");
    let apps = AppLoader::load();
    println!("********* 检索完毕 *********");
    if apps.is_empty() {
        println!("没有找到应用程序。。。");
        return;
    }
    let search_engine = SearchEngine::new(apps);
    let launcher = Launcher::new(search_engine);
    launcher.run();
    println!("程序结束");
}
