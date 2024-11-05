pub fn add_string_multhiple_n(str: &str, number: u8) -> String {
    let mut v: Vec<String> = Vec::new();

    for _ in 0..number {
        v.push(str.to_string().clone());
    }

    let mut result = String::new();
    for e_str in v {
        result.push_str(&e_str);
    }

    result
}