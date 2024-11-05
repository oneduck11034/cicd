// https://school.programmers.co.kr/learn/courses/30/lessons/181942
pub fn put_write_one_two(str1: &str, str2: &str) -> String {
    let str1_chars_v: Vec<char> = str1.chars().collect();
    let str2_chars_v: Vec<char> = str2.chars().collect();
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