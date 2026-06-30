use std::process::{Command, Stdio};

pub struct AppRunner;

impl AppRunner {

    /// `run` 启动应用程序
    ///
    /// # Examples
    /// ```
    /// use jian_box::AppRunner;
    /// AppRunner::run("Exec=/usr/bin/wechat %U");
    /// ```
    pub fn run(exec: &str) {
        println!("打开的文件：{}", exec);
        let parts: Vec<&str> = exec.split_whitespace().collect();
        let Some(cmd) = parts.first() else {
            println!("exec为空，无法执行打开操作");
            return;
        };

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
