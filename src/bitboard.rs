use std::fmt::Display;

pub struct BitBoard(u64);

impl BitBoard {
    pub fn new(val: u64) -> Self {
        BitBoard(val)
    }
}

impl Default for BitBoard {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl Display for BitBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        for (i, c) in format!("{:064b}", self.0).chars().enumerate() {
            if i % 8 == 0 && i != 0 {
                result.push('\n');
            }

            result.push_str(&format!("{} ", c));
        }
        write!(f, "{}", result)
    }
}
