
/**
 * Receive an i32 vector containing all digits found in a str.
 * If no digit is found, the vector is empty.
 * Each character is handled individually. 
 * So 123ab456 will result in [1, 2, 3, 4, 5, 6]
 */

pub trait RetrieveDigits {
    fn retrieve_digits(&self) -> Vec<i32>;
}

impl RetrieveDigits for &str {
    fn retrieve_digits(&self) -> Vec<i32> {
        let mut ints: Vec<i32> = Vec::new();
        for char in self.chars() {
            if let Some(digit) = char.to_digit(10) {
                ints.push(digit as i32);
            }
        }

        ints
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
mod tests_retrieve_digits {
    use super::*;

    #[test]
    fn test_empty_string() {
        let s = "";
        let digits = s.retrieve_digits();
        assert!(digits.is_empty());
    }

    #[test]
    fn test_no_digits() {
        let s = "abc";
        let digits = s.retrieve_digits();
        assert!(digits.is_empty());
    }

    #[test]
    fn test_only_digits() {
        let s = "123456";
        let digits = s.retrieve_digits();
        assert_eq!(digits, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_mixed_content() {
        let s = "a1b2c3";
        let digits = s.retrieve_digits();
        assert_eq!(digits, vec![1, 2, 3]);
    }
}

#[cfg(test)]
mod tests_retrieve_ints {
    use super::*;

    #[test]
    fn test_empty_string() {
        let s = "";
        let ints = s.retrieve_ints();
        assert!(ints.is_empty());
    }

    #[test]
    fn test_no_digits() {
        let s = "abc";
        let ints = s.retrieve_ints();
        assert!(ints.is_empty());
    }

    #[test]
    fn test_only_digits() {
        let s = "123456";
        let ints = s.retrieve_ints();
        assert_eq!(ints, vec![123456]);
    }

    #[test]
    fn test_mixed_content() {
        let s = "123ab456";
        let ints = s.retrieve_ints();
        assert_eq!(ints, vec![123, 456]);
    }

    #[test]
    #[should_panic(expected = "Number can not be parsed into a i32")]
    fn test_overflow() {
        let s = "2147483648"; // One more than i32::MAX
        s.retrieve_ints();
    }
}
