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

use std::io;
use std::str::FromStr;

#[allow(dead_code)]
fn read_line() -> String {
    let stdin = io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).ok();
    buf
}

#[allow(dead_code)]
fn read_vec<T>() -> Vec<T> where T: FromStr {
    let line = read_line();
    line.split_whitespace()
        .map(|item| T::from_str(item).ok().unwrap())
        .collect::<Vec<T>>()
}

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [(i32, i32, i32); n],
    }
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


