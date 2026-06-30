use std::fs::read_to_string;

#[test]
fn test_read_desktop() {
    let file_path = String::from("/usr/share/applications/google-chrome.desktop");
    let content = read_to_string(file_path).unwrap();
    let sections = content.split("[Desktop");
    let ss: String = sections.filter(|s| s.starts_with(" Entry]")).collect();
    println!("{}", ss);
}

fn fuzzy_match(text: &str, pattern: &str) -> bool {
    if pattern.is_empty() {
        return false;
    }

    let mut pattern_chars = pattern.chars();
    let mut cur_pattern_char = pattern_chars.next();

    for char in text.chars() {
        if let Some(p_char) = cur_pattern_char {
            if char == p_char {
                cur_pattern_char = pattern_chars.next();
            }
        } else {
            return true;
        }
    }
    cur_pattern_char.is_none()
}

#[test]
fn test_fuzzy_match() {
    let name = "WeChat".to_lowercase();
    println!("{}", fuzzy_match(&name, "we"));
    println!("{}", fuzzy_match(&name, "wc"));
    println!("{}", fuzzy_match(&name, "wt"));
    println!("{}", fuzzy_match(&name, "ca"));
    println!("{}", fuzzy_match(&name, "tw"));
}

#[test]
fn test_cut_desktop_content() {
    let content = read_to_string("/usr/share/applications/wechat.desktop").unwrap();
    let exec_content = content.lines().find(|s| s.starts_with("Exec"));
    if let Some(exec_content) = exec_content {
        let ss = exec_content.strip_prefix("Exec=");
        println!("{:?}", ss);
    }
}
