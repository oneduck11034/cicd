mod q1;
#[cfg(test)]
mod tests_a {
    use super::*;

    #[test]
    fn it_works() {
        let result = q1::add_string_multhiple_n("string", 5);
        assert_eq!(result, "stringstringstringstringstring".to_string());
    }
}

mod q2;

#[cfg(test)]
mod tests_b {
    use super::*;

    #[test]
    fn it_works() {
        let result = q2::string_add_write("He11oWor1d", "lloWorl", 2);
        assert_eq!(result, "HelloWorld".to_string());
    }

    #[test]
    fn it_works2() {
        let result = q2::string_add_write("Program29b8UYP", "merS123",	7);
        assert_eq!(result, "ProgrammerS123".to_string());
    }
}


mod q3;
#[cfg(test)]
mod tests_c {
    use super::*;

    #[test]
    fn it_works() {
        let result = q3::put_write_one_two("aaaaa", "bbbbb");
        assert_eq!(result, "ababababab".to_string());
    }
}



mod q4;
// ---- tests_d::it_works_d1 stdout ----
// thread 'tests_d::it_works_d1' panicked at /usr/src/debug/rust/rustc-1.82.0-src/library/core/src/num/mod.rs:1136:5:
// attempt to multiply with overflow
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

// ---- tests_d::it_works_d2 stdout ----
// thread 'tests_d::it_works_d2' panicked at /usr/src/debug/rust/rustc-1.82.0-src/library/core/src/num/mod.rs:1136:5:
// attempt to multiply with overflow⏎  
#[cfg(test)]
mod tests_d {
    use super::*;

    #[test]
    fn it_works_d1() {
        let result = q4::q4(Vec::from([3_u32, 4, 5, 2, 1]));
        assert_eq!(result, 1);
    }

    #[test]
    fn it_works_d2() {
        let result = q4::q4(Vec::from([5_u32, 7, 8, 3]));
        assert_eq!(result, 0);
    }
}


mod q5;
#[cfg(test)]
mod tests_e {
    use super::*;

    #[test]
    fn it_works_e1() {
        q5::something();
        // assert_eq!(result, 1);
    }
}