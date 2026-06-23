use std::collections::HashSet;
use std::env;
use std::fs::{read_dir, read_to_string};
use std::io::{Write, stdin, stdout};
use std::process::{Command, Stdio};

const PATH: &str = "/usr/share/applications";

#[derive(Debug)]
struct App {
    apps: Vec<AppEntry>,
}
#[derive(Debug)]
struct AppEntry {
    id: u32,
    name: String,
    exec: String,
    desktop_file: String,
    search_key: String,
    comment: String,
    icon: String,
}

fn main() {
    println!("********* 开始检索desktop *********");
    let app = match process_desktop_files(PATH) {
        Ok(app) => app,
        Err(err) => panic!("出现错误: {}", err),
    };
    println!("{:#?}", app);
    println!("********* 检索完毕 *********");

    // 搜索
    search_handler(&app);
}

/// `process_desktop_files` 读取电脑desktop信息
///
/// # Examples
///
/// ```
/// const PATH: &str = "/usr/share/applications";
/// let apps = process_desktop_files(PATH).unwrap();
/// ```
///
fn process_desktop_files(path: &str) -> Result<App, Box<dyn std::error::Error>> {
    let entries = read_dir(path)?;
    let mut apps = Vec::with_capacity(30);
    let lang = get_preferred_language().unwrap_or_else(|| panic!("获取当前系统语言失败"));
    let cur_env_name = format!("Name[{}]=", lang);
    let cur_env_comment = format!("Comment[{}]=", lang);

    for (i, entry) in entries.enumerate() {
        let mut app_entry = AppEntry {
            id: 0,
            name: String::new(),
            exec: String::new(),
            desktop_file: String::new(),
            search_key: String::new(),
            comment: String::new(),
            icon: String::new(),
        };

        let entry = entry?;
        let path = entry.path();
        // 只处理 desktop 结尾的文件
        if let Some(ext) = path.extension() {
            if ext != "desktop" {
                continue;
            }
        }

        let filename = path.display().to_string();
        let content = read_to_string(&filename)?;

        // 过滤隐藏的desktop
        if let Some(no_display) = content
            .lines()
            .find(|line| line.contains("NoDisplay=true"))
            .map(|line| !line.is_empty())
        {
            if no_display {
                continue;
            }
        }

        // 获取名称
        let mut names: HashSet<String> = HashSet::new();
        let mut is_find_env_name = false;
        for line in content.lines() {
            if line.starts_with("Name") {
                if !is_find_env_name {
                    // 获取当前环境语言的名称
                    if line.starts_with(&cur_env_name) {
                        app_entry.name = line.replace(&cur_env_name, "");
                        is_find_env_name = true;
                    } else if line.starts_with("Name=") {
                        // 获取默认名称
                        app_entry.name = line.replace("Name=", "");
                    }
                }
                // 将全部名称作为搜索词
                let mut name = line.split("=");
                name.next();
                let name = name.next().unwrap_or_else(|| panic!("获取名称失败"));
                names.insert(format!("{},", name));
            }
        }
        let mut names: String = names.into_iter().collect();
        // 去掉尾部的逗号
        names.remove(names.len() - 1);
        app_entry.search_key = names;

        let exec = content
            .lines()
            .find(|line| line.starts_with("Exec"))
            .map(|line| line.to_string());
        if let Some(exec) = exec {
            app_entry.exec = exec.replace("Exec=", "");
        } else {
            continue;
        }

        if let Some(icon) = content
            .lines()
            .find(|line| line.starts_with("Icon"))
            .map(|line| line.to_string())
        {
            app_entry.icon = icon.replace("Icon=", "");
        }

        for line in content.lines() {
            if line.starts_with("Comment") {
                if line.starts_with(&cur_env_comment) {
                    app_entry.comment = line.replace(&cur_env_comment, "");
                    break;
                } else {
                    if line.starts_with("Comment=") {
                        app_entry.comment = line.replace("Comment=", "");
                    }
                }
            }
        }

        app_entry.id = (i + 1) as u32;
        app_entry.desktop_file = filename;

        apps.push(app_entry);
    }
    Ok(App { apps })
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
fn search(app: &App, query: String) -> Option<Vec<&AppEntry>> {
    let mut results = Vec::with_capacity(10);
    for apps in &app.apps {
        let result = apps
            .search_key
            .to_lowercase()
            .find(query.to_lowercase().as_str());
        if let Some(_) = result {
            results.push(apps);
        }
    }
    Some(results)
}

/// `get_preferred_language` 从环境变量中获取当前系统语言
///
/// # Examples
///
/// ```
///   let lang = get_preferred_language();
///   assert_eq!("zh_CN",lang.unwrap());
/// ````
fn get_preferred_language() -> Option<String> {
    env::var("LANG")
        .ok()
        .and_then(|lang| lang.split('.').next().map(|s| s.to_string()))
        .or_else(|| env::var("LC_MESSAGES").ok())
}

/// `search_handler` 处理搜索
///
/// ```
/// const PATH: &str = "/usr/share/applications";
/// let app = process_desktop_files(PATH).unwrap();
/// search_handler(&app);
/// ```
fn search_handler(app: &App) {
    loop {
        print!("请输入desktop名称（输入exit结束程序）：");
        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).expect("读取失败");
        let input = input.trim();
        if input.is_empty() {
            println!("输入的内容为空");
            continue;
        }
        if input == "exit" {
            break;
        }
        let results = search(&app, String::from(input));
        if let Some(results) = results {
            if results.is_empty() {
                println!("没有查询到程序，请重新输入~");
                continue;
            }
            for (i, result) in results.iter().enumerate() {
                println!(
                    "序号:{}\t名称：{}\t说明：{}",
                    i + 1,
                    result.name,
                    result.comment
                )
            }
            println!("查询到的数量：{}", results.len());
            let mut application = None;
            loop {
                println!("请选择要打开的应用（序号，0=退出）：");
                let mut index = String::new();
                stdin().read_line(&mut index).expect("读取失败");
                let index: usize = index.trim().parse().unwrap();
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
                        println!("无效的选择");
                        continue;
                    }
                };
                break;
            }
            let application = match application {
                Some(app) => app,
                None => continue,
            };
            println!("打开的文件：{}", application.exec);
            let parts: Vec<&str> = application.exec.split_whitespace().collect();
            let cmd = parts[0];

            // 解析占位符（去除 %U，%F，……）
            let args: Vec<&str> = parts[1..]
                .iter()
                .filter(|arg| !arg.contains('%'))
                .copied()
                .collect();

            match Command::new(cmd)
                .args(&args)
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
            {
                Ok(child) => {
                    println!("✅ 启动成功：{}", child.id());
                }
                Err(err) => {
                    eprintln!("❌ 启动失败：{}", err);
                }
            }
        }
    }
    println!("程序结束");
}
