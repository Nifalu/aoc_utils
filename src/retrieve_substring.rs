pub trait RetrieveSubstring {
    fn find_positions(&self, strings: Vec<&str>) -> Vec<Vec<(i32, String)>>;
}

impl RetrieveSubstring for &str {
    fn find_positions(&self, strings: Vec<&str>) -> Vec<Vec<(i32, String)>> {
        let mut positions = Vec::new();
    
        for s in strings.iter() {
            let mut current_positions = Vec::new();
            let mut start = 0;
    
            while let Some(found) = self[start..].find(s) {
                let index = (start + found) as i32;
                current_positions.push((index, s.to_string()));
                start = start + found + 1; // Move just past the found index
            }
    
            positions.push(current_positions);
        }
    
        positions
    }
}