
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