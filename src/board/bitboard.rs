use std::{
    fmt::Display,
    ops::{BitAnd, BitOr},
};

#[derive(Debug, Clone, Copy)]
pub struct BitBoard(u64);

impl BitBoard {
    pub const fn new(val: u64) -> Self {
        BitBoard(val)
    }
}

impl Default for BitBoard {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl BitAnd for BitBoard {
    type Output = BitBoard;

    #[inline(always)]
    fn bitand(self, other: Self) -> Self::Output {
        BitBoard(self.0 & other.0)
    }
}

impl BitOr for BitBoard {
    type Output = BitBoard;

    #[inline(always)]
    fn bitor(self, other: Self) -> Self::Output {
        BitBoard(self.0 | other.0)
    }
}

impl Display for BitBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        for (i, c) in format!("{:064b}", self.0).chars().enumerate() {
            if i % 8 == 0 && i != 0 {
                output.push('\n');
            }

            output.push_str(&format!("{} ", c));
        }
        write!(f, "{}", output)
    }
}
