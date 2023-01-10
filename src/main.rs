// -*- coding:utf-8-unix -*-
// Title: 
// Problem URL: 
// Editorial: 
// Solution: 
//


// Limitation:
//
// 
// Memo:


mod input {
    use std::io;
    use std::str::FromStr;

    #[allow(dead_code)]
    pub fn read_line() -> String {
        let stdin = io::stdin();
        let mut buf = String::new();
        stdin.read_line(&mut buf).ok();
        buf
    }

    #[allow(dead_code)]
    pub fn read_val<T>() -> T where T: FromStr {
        let line = read_line();
        let line = line.trim();
        T::from_str(line).ok().unwrap()
    }

    #[allow(dead_code)]
    pub fn read_vec<T>() -> Vec<T> where T: FromStr {
        let line = read_line();
        line.split_whitespace()
            .map(|item| T::from_str(item).ok().unwrap())
            .collect::<Vec<T>>()
    }

    #[allow(dead_code)]
    pub fn read_mat<T>(line: usize) -> Vec<Vec<T>> where T: FromStr {
        let mut ret: Vec<Vec<T>> = Vec::new();
        for _ in 0..line {
            ret.push(read_vec());
        }
        ret
    }
}

mod output {
    // https://stackoverflow.com/a/30325430

    use std::fmt;

    pub(super) struct OutputVec<'a, T> {
        slice: &'a [T],
        separator: &'a str,
    }

    impl<'a, T> OutputVec<'a, T> {
        pub fn new(slice: &'a [T], separator: &'a str) -> Self {
            Self { slice, separator }
        }
    }

    impl<'a, T> fmt::Display for OutputVec<'a, T> where T: fmt::Display {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let mut buf = String::new();
            for item in &self.slice[0..self.slice.len()-1] {
                buf.push_str(&format!("{}" , item));
                buf.push_str(self.separator);
            }
            buf.push_str(&format!("{}", self.slice[self.slice.len()-1]));
            write!(f, "{}", buf)
        }
    }
}

// main
fn main() {
    let _n = input::read_val::<usize>();

}



// #[cfg(test)]
// mod test {
//     use super::*;
//     use proconio::source::auto::AutoSource;
// 
//     #[test]
//     fn test_() {
//         testcase("");
//     }
// 
//     fn testcase(input: &str, ) {
//         let source = AutoSource::from(input);
//         input! {
//             from source,
//         }
//         let ans = solve();
//         assert_eq!(ans, );
//     }
// }


