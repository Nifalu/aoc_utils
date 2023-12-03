
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
    fn retrieve_max_digit(&self) -> Option<i32>;
    fn retrieve_min_digit(&self) -> Option<i32>;
    fn retrieve_first_digit(&self) -> Option<i32>;
    fn retrieve_last_digit(&self) -> Option<i32>;
}

impl RetrieveDigits for &str {
    fn retrieve_all_digits(&self) -> Vec<i32> {
        filter_i32(self)
     }

     fn retrieve_nth_digit(&self, n: usize) -> Option<i32> {
        self.retrieve_all_digits().get(n).copied()
     }
    
     fn retrieve_digit_at_pos(&self, n: usize) -> Option<i32> {
        self.chars()
        .nth(n)
        .and_then(|c| c.to_digit(10))
        .map(|d| d as i32)
     }
    
     fn retrieve_max_digit(&self) -> Option<i32> {
        filter_i32(self).iter().max().copied()
     }
    
     fn retrieve_min_digit(&self) -> Option<i32> {
        filter_i32(self).iter().min().copied()
     }
    
     fn retrieve_first_digit(&self) -> Option<i32> {
        filter_i32(self).first().copied()
     }
    
     fn retrieve_last_digit(&self) -> Option<i32> {
        filter_i32(self).last().copied()
     }
}

fn filter_i32(s : &str) -> Vec<i32> {
    s.chars()
    .filter_map(|c| c.to_digit(10))
    .map(|d| d as i32)
    .collect()
}
/**
 * Receive an i32 vector containing all numbers found in a str
 * If no digit is found, the vector is empty.
 * Consecutive digits will be parsed into a single integer.
 * So "123ab456" will result in [123, 456]
 */
pub trait RetrieveInts {
    fn retrieve_all_ints(&self) -> Vec<i32>;
    fn retrieve_nth_int(&self, n: usize) -> Option<i32>;
    fn retrieve_int_at_pos(&self, n: usize) -> Option<i32>;
    fn retrieve_max_int(&self) -> Option<i32>;
    fn retrieve_min_int(&self) -> Option<i32>;
    fn retrieve_first_int(&self) -> Option<i32>;
    fn retrieve_last_int(&self) -> Option<i32>;
}

impl RetrieveInts for &str {
    fn retrieve_all_ints(&self) -> Vec<i32> {
        let mut ints = Vec::new();
        let mut current_num = String::new();

        for c in self.chars() {
            if c.is_digit(10) {
                current_num.push(c);
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

    fn retrieve_nth_int(&self, n: usize) -> Option<i32> {
        self.retrieve_all_ints().get(n).copied()
    }

    fn retrieve_int_at_pos(&self, n: usize) -> Option<i32> {
        if n >= self.len() {
            return None
        }

        // Find the start of the number
        let mut start = n;
        while start > 0 && self.chars().nth(start - 1).unwrap_or('a').is_digit(10) {
            start -= 1;
        }

        let mut curr_num = String::new();

        // Add everthing until the end of the number
        for c in self[start..].chars() {
            if c.is_digit(10) {
                curr_num.push(c);
            } else {
                break;
            }
        }
        if curr_num.is_empty() {
            None
        } else {
            curr_num.parse::<i32>().ok()
        }
    }

    fn retrieve_max_int(&self) -> Option<i32> {
        self.retrieve_all_ints().iter().max().copied()
    }

    fn retrieve_min_int(&self) -> Option<i32> {
        self.retrieve_all_ints().iter().min().copied()
    }

    fn retrieve_first_int(&self) -> Option<i32> {
        self.retrieve_all_ints().iter().next().copied()
    }

    fn retrieve_last_int(&self) -> Option<i32> {
        self.retrieve_all_ints().iter().last().copied()
    }
}

impl RetrieveInts for Vec<char> {
    fn retrieve_all_ints(&self) -> Vec<i32> {
        let mut ints = Vec::new();
        let mut current_num = String::new();

        for c in self {
            if c.is_digit(10) {
                current_num.push(*c);
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

    fn retrieve_nth_int(&self, n: usize) -> Option<i32> {
        self.retrieve_all_ints().get(n).copied()
    }

    fn retrieve_int_at_pos(&self, n: usize) -> Option<i32> {
        if n >= self.len() {
            return None
        }

        // Find the start of the number
        let mut start = n;
        while start > 0 && self.get(start-1).map_or(false, |&c| c.is_digit(10)) {
            start -= 1;
        }

        let mut curr_num = String::new();

        // Add everthing until the end of the number
        for &c in &self[start..] {
            if c.is_digit(10) {
                curr_num.push(c);
            } else {
                break;
            }
        }
        if curr_num.is_empty() {
            None
        } else {
            curr_num.parse::<i32>().ok()
        }
    }

    fn retrieve_max_int(&self) -> Option<i32> {
        self.retrieve_all_ints().iter().max().copied()
    }

    fn retrieve_min_int(&self) -> Option<i32> {
        self.retrieve_all_ints().iter().min().copied()
    }

    fn retrieve_first_int(&self) -> Option<i32> {
        self.retrieve_all_ints().iter().next().copied()
    }

    fn retrieve_last_int(&self) -> Option<i32> {
        self.retrieve_all_ints().iter().last().copied()
    }
}



/*--------------------------- TESTS ---------------------------*/

#[cfg(test)]
mod retrieve_digits_tests {
    use super::*;

    #[test]
    fn test_retrieve_all_digits() {
        assert_eq!("123ab456".retrieve_all_digits(), vec![1, 2, 3, 4, 5, 6]);
        assert_eq!("abc".retrieve_all_digits(), Vec::<i32>::new());
        assert_eq!("".retrieve_all_digits(), Vec::<i32>::new());
    }

    #[test]
    fn test_retrieve_nth_digit() {
        assert_eq!("12345".retrieve_nth_digit(2), Some(3));
        assert_eq!("12345".retrieve_nth_digit(5), None);
    }

    #[test]
    fn test_retrieve_digit_at_pos() {
        assert_eq!("123ab456".retrieve_digit_at_pos(0), Some(1));
        assert_eq!("123ab456".retrieve_digit_at_pos(3), None);
        assert_eq!("abc".retrieve_digit_at_pos(1), None);
    }

    #[test]
    fn test_retrieve_max_digit() {
        assert_eq!("12983475".retrieve_max_digit(), Some(9));
        assert_eq!("abc".retrieve_max_digit(), None);
    }

    #[test]
    fn test_retrieve_min_digit() {
        assert_eq!("12983475".retrieve_min_digit(), Some(1));
        assert_eq!("abc".retrieve_min_digit(), None);
    }

    #[test]
    fn test_retrieve_first_digit() {
        assert_eq!("a123".retrieve_first_digit(), Some(1));
        assert_eq!("abc".retrieve_first_digit(), None);
    }

    #[test]
    fn test_retrieve_last_digit() {
        assert_eq!("123b".retrieve_last_digit(), Some(3));
        assert_eq!("abc".retrieve_last_digit(), None);
    }
}


#[cfg(test)]
mod retrieve_ints_tests {
    use super::*;

    #[test]
    fn test_retrieve_all_ints() {
        assert_eq!("123ab456".retrieve_all_ints(), vec![123, 456]);
        assert_eq!("abc".retrieve_all_ints(), Vec::<i32>::new());
        assert_eq!("".retrieve_all_ints(), Vec::<i32>::new());
    }

    #[test]
    fn test_retrieve_nth_int() {
        assert_eq!("12345 678".retrieve_nth_int(1), Some(678));
        assert_eq!("12345".retrieve_nth_int(2), None);
    }

    #[test]
    fn test_retrieve_int_at_pos() {
        assert_eq!("ab123 456cd".retrieve_int_at_pos(2), Some(123));
        assert_eq!("ab123 456cd".retrieve_int_at_pos(6), Some(456));
        assert_eq!("abc".retrieve_int_at_pos(1), None);
    }

    #[test]
    fn test_retrieve_max_int() {
        assert_eq!("123 456 789".retrieve_max_int(), Some(789));
        assert_eq!("abc".retrieve_max_int(), None);
    }

    #[test]
    fn test_retrieve_min_int() {
        assert_eq!("123 456 789".retrieve_min_int(), Some(123));
        assert_eq!("abc".retrieve_min_int(), None);
    }

    #[test]
    fn test_retrieve_first_int() {
        assert_eq!("123 456".retrieve_first_int(), Some(123));
        assert_eq!("abc".retrieve_first_int(), None);
    }

    #[test]
    fn test_retrieve_last_int() {
        assert_eq!("123 456".retrieve_last_int(), Some(456));
        assert_eq!("abc".retrieve_last_int(), None);
    }
}
