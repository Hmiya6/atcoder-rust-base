// -*- coding:utf-8-unix -*-
// Title: 
// Problem: 
// Editorial: 
// Solution: 
//


// Limitation:
//
// 
// Memo:
//
//

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [(i32, i32, i32); n],
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use proconio::source::auto::AutoSource;

    #[test]
    fn test_() {
        testcase("");
    }

    fn testcase(input: &str, ) {
        let source = AutoSource::from(input);
        input! {
            from: source,
        }
        let ans = solve();
        assert_eq!(ans, );
    }
}
