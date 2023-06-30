use crate::string::prefix_function::prefix_function;

/// O(m + n) time.
///
/// O(m) space.
pub fn kmp(pattern: &str, text: &str) -> Vec<usize> {
    let pattern_len = pattern.len();
    let text_len = text.len();

    let mut answer = Vec::new();

    if pattern_len > text_len {
        return answer;
    }

    // O(m) space
    let prefix_array = prefix_function(&(pattern.to_owned() + "#"));
    let mut curr = prefix_array[pattern_len]; // 0

    // prefix_array[prev] <= pattern_len
    let get_next = |mut prev: usize, index: usize| -> usize {
        while prev > 0 && text.chars().nth(index) != pattern.chars().nth(prev) {
            prev = prefix_array[prev - 1];
        }

        if text.chars().nth(index) == pattern.chars().nth(prev) {
            prev += 1;
        }

        prev
    };

    for i in 0..text_len {
        curr = get_next(curr, i);

        if curr == pattern_len {
            answer.push(i + 1 - pattern_len);
        }
    }

    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kmp_unit() {
        let pattern = "aba";
        let text = "babac";
        assert_eq!(kmp(pattern, text), [1]);
    }
}
