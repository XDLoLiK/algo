use std::{cmp::min, vec::Vec};

/// O(string.len()) time.
pub fn z_function(string: &str) -> Vec<usize> {
    let strlen = string.len();
    let mut z_array = vec![0; strlen];

    let mut left = 0;
    let mut right = 0;

    for i in 1..strlen {
        if i <= right {
            z_array[i] = min(right - i + 1, z_array[i - left]);
        }

        while i + z_array[i] < strlen
            && string.chars().nth(z_array[i]) == string.chars().nth(i + z_array[i])
        {
            z_array[i] += 1;
        }

        if i + z_array[i] - 1 > right {
            left = i;
            right = i + z_array[i] - 1;
        }
    }

    z_array
}
