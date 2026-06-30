use anyhow::Context;
use jian_box::*;

#[test]
fn test_app_loader_integration() {
    let config = Config::load().context("Failed to load config").unwrap();
    // 测试从实际目录加载
    let apps = AppLoader::load(&config);
    assert!(!apps.is_empty(), "加载应用程序完毕");
    println!("{:#?}", apps);
}
