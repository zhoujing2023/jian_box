use crate::app_runner::AppRunner;
use crate::search_engine::SearchEngine;
use std::io::{stdin, stdout, Write};

pub struct Launcher {
    search_engine: SearchEngine,
}

impl Launcher {
    /// `new` 创建实例
    pub fn new(search_engine: SearchEngine) -> Self {
        Self { search_engine }
    }

    /// `run` 处理搜索
    ///
    /// # Examples
    /// ```
    /// const PATH: &str = "/usr/share/applications";
    /// let app = process_desktop_files(PATH).unwrap();
    /// search_handler(&app);
    /// ```
    pub fn run(&self) {
        loop {
            print!("请输入desktop名称（输入exit结束程序）：");
            stdout().flush().unwrap();
            let mut input = String::new();
            match stdin().read_line(&mut input) {
                Ok(_) => (),
                Err(err) => {
                    eprintln!("读取输入的内容失败：{}", err);
                    println!("请重新输入~");
                    continue;
                }
            }
            let input = input.trim();
            if input.is_empty() {
                println!("输入的内容为空");
                continue;
            }
            if input == "exit" {
                break;
            }
            let results = self.search_engine.search(input);
            if results.is_empty() {
                println!("没有查询到程序，请重新输入~");
                continue;
            }
            for (i, result) in results.iter().enumerate() {
                println!(
                    "序号:{}\t名称：{}\t分数：{}\t说明：{}",
                    i + 1,
                    result.name,
                    result.score.borrow(),
                    result.comment,
                )
            }
            println!("查询到的数量：{}", results.len());
            let mut application = None;
            loop {
                println!("请选择要打开的应用（序号，0=退出）：");
                let mut index = String::new();
                stdin().read_line(&mut index).expect("读取失败");
                let index: usize = match index.trim().parse() {
                    Ok(num) => num,
                    Err(err) => {
                        eprintln!("转换失败，异常信息：{}", err);
                        println!("请重新输入～");
                        continue;
                    }
                };
                if index == 0 {
                    break;
                }
                if index < 1 {
                    println!("序号不能小于1，请重新选择~");
                    continue;
                }
                let app = results.get(index - 1);
                application = match app {
                    Some(app) => Some(app),
                    None => {
                        println!("无效的选择~");
                        continue;
                    }
                };
                break;
            }
            let application = match application {
                Some(app) => app,
                None => continue,
            };
            *application.score.borrow_mut() += 1;
            // 更新分数
            self.search_engine.persistent_usage();
            // 打开应用程序
            AppRunner::run(&application.exec);
        }
    }
}
