use std::vec::Vec;

/// O(string.len()) time.
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
