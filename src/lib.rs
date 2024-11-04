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

#[cfg(test)]
mod tests_a {
    use super::*;

    #[test]
    fn it_works() {
        let result = add_string_multhiple_n("string", 5);
        assert_eq!(result, "stringstringstringstringstring".to_string());
    }
}


// https://school.programmers.co.kr/learn/courses/30/lessons/181942
pub fn string_add_write(my_string: &str, overwrite_string: &str, s: usize) -> String {
    let mut result= String::new();
    let overwrite_len= overwrite_string.len();
    let frist_string= String::from(&my_string[0..s]);
    let latsted_len= s + overwrite_len;
    let last_string= &my_string[latsted_len..my_string.len()];

    result= format!("{}{}{}", frist_string, overwrite_string, last_string);

    result
}

#[cfg(test)]
mod tests_b {
    use super::*;

    #[test]
    fn it_works() {
        let result = string_add_write("He11oWor1d", "lloWorl", 2);
        assert_eq!(result, "HelloWorld".to_string());
    }

    #[test]
    fn it_works2() {
        let result = string_add_write("Program29b8UYP", "merS123",	7);
        assert_eq!(result, "ProgrammerS123".to_string());
    }
}