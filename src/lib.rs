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


// https://school.programmers.co.kr/learn/courses/30/lessons/181942
pub fn put_write_one_two(str1: &str, str2: &str) -> String {
    let mut str1_chars_v: Vec<char> = str1.chars().collect();
    let mut str2_chars_v: Vec<char> = str2.chars().collect();
    let mut result_v: Vec<char>= Vec::new();
    let mut flag= false;
    let mut idx= 0_usize;
    // str1_chars_v.append(&mut str2_chars_v);
    // left: "aaaaabbbbb" 

    loop {
        if flag {
            let c= str2_chars_v[idx];
            result_v.push(c);
            flag= false;
        }if flag == false{
            let c= str1_chars_v[idx];
            result_v.push(c);
            flag= true;
        }

        idx+=1;
        if idx >= str1_chars_v.len() {
            break;
        }
    }
//   left: "ababababa"

    let result: String = result_v.iter().collect();

    result
}

#[cfg(test)]
mod tests_c {
    use super::*;

    #[test]
    fn it_works() {
        let result = put_write_one_two("aaaaa", "bbbbb");
        assert_eq!(result, "ababababab".to_string());
    }
}



// ---- tests_d::it_works_d1 stdout ----
// thread 'tests_d::it_works_d1' panicked at /usr/src/debug/rust/rustc-1.82.0-src/library/core/src/num/mod.rs:1136:5:
// attempt to multiply with overflow
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

// ---- tests_d::it_works_d2 stdout ----
// thread 'tests_d::it_works_d2' panicked at /usr/src/debug/rust/rustc-1.82.0-src/library/core/src/num/mod.rs:1136:5:
// attempt to multiply with overflow⏎  
pub fn q4(num_v: Vec<u32>) -> i32 {
    let sumed: u32= num_v.iter().sum();
    let powed: u32= sumed.pow(sumed);
    // let powed= i32::pow(sumed, sumed).unwrap();

    let mut multi= 1_u32;
    for num_e in num_v {
        multi*= num_e;
    }

    if multi < powed {
        return 1_i32;
    }else{
        return 0_i32;
    }
}

#[cfg(test)]
mod tests_d {
    use super::*;

    #[test]
    fn it_works_d1() {
        let result = q4(Vec::from([3_u32, 4, 5, 2, 1]));
        assert_eq!(result, 1);
    }

    #[test]
    fn it_works_d2() {
        let result = q4(Vec::from([5_u32, 7, 8, 3]));
        assert_eq!(result, 0);
    }
}