/// O(n) time.
pub fn prefix_function(string: &str) -> Vec<usize> {
    let strlen = string.len();
    let mut prefix_array = vec![0; strlen];

    for i in 1..strlen {
        let mut prev = prefix_array[i - 1];

        while prev > 0 && string.chars().nth(i) != string.chars().nth(prev) {
            prev = prefix_array[prev - 1];
        }

        if string.chars().nth(i) == string.chars().nth(prev) {
            prev += 1;
        }

        prefix_array[i] = prev;
    }

    prefix_array
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prefix_function_unit_1() {
        let string = "aaaaa";
        assert_eq!(prefix_function(string), [0, 1, 2, 3, 4]);
    }

    #[test]
    fn prefix_function_unit_2() {
        let string = "aabaaab";
        assert_eq!(prefix_function(string), [0, 1, 0, 1, 2, 2, 3]);
    }

    #[test]
    fn prefix_function_unit_3() {
        let string = "abcabcd";
        assert_eq!(prefix_function(string), [0, 0, 0, 1, 2, 3, 0]);
    }
}
