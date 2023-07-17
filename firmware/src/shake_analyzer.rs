use crate::ring::Ring;
use heapless::Vec;

pub struct ShakeAnalyzer {
    data: Ring<100, i16>,
}

impl ShakeAnalyzer {
    pub fn new() -> ShakeAnalyzer {
        ShakeAnalyzer {
            data: Ring::<100, i16>::new(),
        }
    }

    pub fn add(&mut self, value: i16) {
        self.data.add(value);
    }

    pub fn get_current_level(&mut self) -> i16 {
        self.data
            .iter()
            .zip(self.data.iter().skip(1))
            .map(|(a, b)| (*a as i32 - *b as i32).abs() / 100)
            .map(|x| x.min(100) as i16)
            .take(5)
            .max()
            .unwrap_or(0)
    }

    pub fn last_values(&mut self) -> Vec<i16, 50> {
        self.data
            .iter()
            .take(50)
            .zip(self.data.iter().take(51).skip(1))
            .map(|(a, b)| (*a as i32 - *b as i32).abs() / 100)
            .map(|x| x.min(100) as i16)
            .collect()
    }
}
