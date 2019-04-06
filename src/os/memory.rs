use std::fmt;

#[derive(Clone)]
pub struct MemoryRange(pub i32, pub i32); // initial and final blocks of memory this process takes up

impl fmt::Display for MemoryRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        for i in self.0..=self.1 {
            result.push_str(&(i.to_string() + " "));
        }
        write!(f, "{}", result)
    }
}
