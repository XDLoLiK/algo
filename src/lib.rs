#![allow(dead_code)]

#[cfg(feature = "string")]
pub mod string;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn z_function_unit() {
        let string = "aaaaa";
        assert_eq!(string::z_function(string), [0, 4, 3, 2, 1]);

        let string = "aaabaab";
        assert_eq!(string::z_function(string), [0, 2, 1, 0, 2, 1, 0]);

        let string = "abacaba";
        assert_eq!(string::z_function(string), [0, 0, 1, 0, 3, 0, 1]);
    }

    #[test]
    fn prefix_function_unit() {
        let string = "aaaaa";
        assert_eq!(string::prefix_function(string), [0, 1, 2, 3, 4]);

        let string = "aabaaab";
        assert_eq!(string::prefix_function(string), [0, 1, 0, 1, 2, 2, 3]);

        let string = "abcabcd";
        assert_eq!(string::prefix_function(string), [0, 0, 0, 1, 2, 3, 0]);
    }

    #[test]
    fn kmp_unit() {
        let pattern = "aba";
        let text = "babac";
        assert_eq!(string::kmp(pattern, text), [1]);
    }
}
