use std::fs::read_to_string;

#[test]
fn test_read_desktop() {
    let file_path = String::from("/usr/share/applications/google-chrome.desktop");
    let content = read_to_string(file_path).unwrap();
    let sections = content.split("[Desktop");
    let ss: String = sections.filter(|s| s.starts_with(" Entry]")).collect();
    println!("{}", ss);
}

