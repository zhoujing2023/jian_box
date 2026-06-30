use anyhow::Context;
use jian_box::{AppLoader, Config, Launcher, SearchEngine};

fn main() -> anyhow::Result<()> {
    let config = Config::load().context("加载配置失败")?;
    println!("加载配置配置信息成功：{:#?}", config);
    println!("********* 开始检索desktop *********");
    let apps = AppLoader::load(&config);
    println!("********* 检索完毕 *********");
    if apps.is_empty() {
        println!("没有找到应用程序。。。");
        return Ok(());
    }
    let search_engine = SearchEngine::new(apps);
    let launcher = Launcher::new(search_engine);
    launcher.run(&config);
    println!("程序结束");
    Ok(())
}
