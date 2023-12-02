
/**
 * Receive an i32 vector containing all digits found in a str.
 * If no digit is found, the vector is empty.
 * Each character is handled individually. 
 * So 123ab456 will result in [1, 2, 3, 4, 5, 6]
 */

pub trait RetrieveDigits {
    fn retrieve_all_digits(&self) -> Vec<i32>;
    fn retrieve_nth_digit(&self, n: usize) -> Option<i32>;
    fn retrieve_digit_at_pos(&self, n: usize) -> Option<i32>;
}

impl RetrieveDigits for &str {
    fn retrieve_all_digits(&self) -> Vec<i32> {
        self.chars()
            .filter_map(|c| c.to_digit(10))
            .map(|d| d as i32)
            .collect()
    }

    fn retrieve_nth_digit(&self, n: usize) -> Option<i32> {
        self.retrieve_all_digits().get(n).copied()
    }

    fn retrieve_digit_at_pos(&self, i: usize) -> Option<i32> {
        self.chars()
            .nth(i)
            .and_then(|c| c.to_digit(10))
            .map(|d| d as i32)
    }
}

/**
 * Receive an i32 vector containing all numbers found in a str
 * If no digit is found, the vector is empty.
 * Consecutive digits will be parsed into a single integer.
 * So "123ab456" will result in [123, 456]
 */
pub trait RetrieveInts {
    fn retrieve_ints(&self) -> Vec<i32>;
}

impl RetrieveInts for &str {
    fn retrieve_ints(&self) -> Vec<i32> {
        let mut ints = Vec::new();
        let mut current_num = String::new();

        for char in self.chars() {
            if char.is_digit(10) {
                current_num.push(char);
            } else if !current_num.is_empty() {
                if let Ok(num) = current_num.parse::<i32>() {
                    ints.push(num);
                } else {
                    panic!("Number can not be parsed into a i32");
                }
                current_num.clear();
            }
        }

        // Check for any remaining number at the end of the string
        if !current_num.is_empty() {
            if let Ok(num) = current_num.parse::<i32>() {
                ints.push(num);
            } else {
                panic!("Number can not be parsed into a i32");
            }
        }

        ints
    }
}




/*--------------------------- TESTS ---------------------------*/


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_retrieve_all_digits() {
        assert_eq!("123ab456".retrieve_all_digits(), vec![1, 2, 3, 4, 5, 6]);
        assert_eq!("abc".retrieve_all_digits(), vec![]);
        assert_eq!("".retrieve_all_digits(), vec![]);
    }

    #[test]
    fn test_retrieve_nth_digit() {
        assert_eq!("123ab456".retrieve_nth_digit(2), Some(3));
        assert_eq!("123ab456".retrieve_nth_digit(6), None);
        assert_eq!("abc".retrieve_nth_digit(0), None);
    }

    #[test]
    fn test_retrieve_digit_at_pos() {
        assert_eq!("123ab456".retrieve_digit_at_pos(2), Some(3));
        assert_eq!("123ab456".retrieve_digit_at_pos(3), None);
        assert_eq!("abc".retrieve_digit_at_pos(0), None);
    }
}

#[cfg(test)]
mod retrieve_ints_tests {
    use super::*;

    #[test]
    fn test_retrieve_ints() {
        assert_eq!("123ab456".retrieve_ints(), vec![123, 456]);
        assert_eq!("123ab45ab6".retrieve_ints(), vec![123, 45, 6]);
        assert_eq!("abc".retrieve_ints(), vec![]);
        assert_eq!("".retrieve_ints(), vec![]);
    }

    // Additional tests can include strings with large numbers, negative numbers (if applicable), 
    // and strings with numbers at the very beginning or end.
}
