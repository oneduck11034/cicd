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
