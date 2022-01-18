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
    pub fn read_vec<T>() -> Vec<T> where T: FromStr {
        let line = read_line();
        line.split_whitespace()
            .map(|item| T::from_str(item).ok().unwrap())
            .collect::<Vec<T>>()
    }
}

// main
fn main() {
    let _n = input::read_vec::<usize>();

}

mod int_utils {

    #[derive(Debug, PartialEq)]
    pub enum ToUintError<'a> {
        NonDigitChar(char),
        OutOfUsize(&'a str),
    }

    use std::fmt;
    impl<'a> fmt::Display for ToUintError<'a> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                Self::NonDigitChar(c) => {
                    write!(f, "Not a digit char: {}", c)
                },
                Self::OutOfUsize(s) => {
                    write!(f, "Out of usize range: {}", s)
                }
            }
        }
    }

    use std::error;
    impl<'a> error::Error for ToUintError<'a> {
        fn description(&self) -> &str {
            match *self {
                Self::NonDigitChar(_c) => {
                    "Not a digit char"
                }
                Self::OutOfUsize(_s) => {
                    "Out of usize range"
                }
            }
        }
    }
    
    // u8, u16, u32, u64, usize に AsRef<str> からの変換を可能にするトレイト
    macro_rules! impl_to_uint {
        ($type:ty, $trait_name:ident, $method_name:ident) => {

            pub trait $trait_name {
                fn $method_name(&self) -> Result<$type, ToUintError>;
            }

            impl<T> $trait_name for T where T: AsRef<str> {
                fn $method_name(&self) -> Result<$type, ToUintError> {
                    use ToUintError::{
                        NonDigitChar,
                        OutOfUsize,
                    };
                    let mut result: $type = 0;
                    let chars = self.as_ref().chars();
                    for item in chars {
                        match item {
                            '0'..='9' => {
                                let n = match item.to_digit(10) {
                                    Some(digit) => { digit as $type },
                                    None => { return Err(NonDigitChar(item)) },
                                };
                                
                                result = result
                                        .checked_mul(10)
                                        .ok_or(OutOfUsize(self.as_ref()))?
                                        .checked_add(n)
                                        .ok_or(OutOfUsize(self.as_ref()))?;
                            }
                            _ => {
                                return Err(NonDigitChar(item));
                            }
                        }
                    }
                    Ok(result)
                }
            }

        }
    }

    impl_to_uint!(u8, ToU8, to_u8);
    impl_to_uint!(u16, ToU16, to_u16);
    impl_to_uint!(u32, ToU32, to_u32);
    impl_to_uint!(u64, ToU64, to_u64);
    impl_to_uint!(usize, ToUsize, to_usize);
    
    #[cfg(test)]
    mod test {
        use super::{ToUsize, ToU64, ToUintError};
        #[test]
        fn str_to_usize() {
            assert_eq!(42, "42".to_usize().unwrap());
        }

        #[test]
        fn string_to_usize() {
            assert_eq!(42, String::from("42").to_usize().unwrap())
        }

        #[test]
        fn u64_test() {
            assert_eq!(42, "42".to_u64().unwrap());
            assert_eq!(42, String::from("42").to_u64().unwrap());
        }
    
        #[test]
        fn not_a_digit_error() {
            assert_eq!(
                ToUintError::NonDigitChar('-'), 
                "-42".to_usize().err().unwrap()
            );
        }

        #[test]
        fn out_of_usize_error() {
            assert_eq!(
                ToUintError::OutOfUsize("999999999999999999999999999999999"), 
                "999999999999999999999999999999999".to_usize().err().unwrap()
            );
        }
        
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


