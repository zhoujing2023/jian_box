#[test]
fn prefix_match() {
    let name_info = "Name=idea";
    if let Some(name) = name_info.strip_prefix("Name=") {
        println!("{}", name);
    }
}

#[test]
fn test_unwrap() {
    let input = "sss";
    let num1: u32 = input.parse().unwrap_or(0); // 如果转换失败则返回 0
    let num2: u32 = input.parse().unwrap_or_else(|_| 0);
    assert_eq!(num1, 0);
    assert_eq!(num2, 0);
}
