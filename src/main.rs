mod app_loader;
mod launcher;
mod model;
mod search_engine;

use crate::app_loader::AppLoader;
use crate::launcher::Launcher;
use crate::search_engine::SearchEngine;

fn main() {
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
