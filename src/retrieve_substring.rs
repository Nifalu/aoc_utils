pub trait RetrieveSubstring {
    fn find_positions(&self, strings: &[&str]) -> Vec<(i32, String)>;
}

impl RetrieveSubstring for &str {
    fn find_positions(&self, strings: &[&str]) -> Vec<(i32, String)> {
        let mut positions = Vec::new();
    
        for s in strings.iter() {
            let mut start = 0;
    
            while let Some(found) = self[start..].find(s) {
                let index = (start + found) as i32;
                positions.push((index, s.to_string()));
                start = start + found + 1; // Move just past the found index
            }
        }
    
        positions
    }    
}