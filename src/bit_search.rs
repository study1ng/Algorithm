/**
 * bit search allows you to search from 000000...0, 00000...1, ..., 11111...1
 */

 pub struct BitSearcherResult {
    n: usize
}

impl BitSearcherResult {
    pub(crate) fn new(n: usize) -> Self {
        Self { n }
    }

    pub fn bits(&self, size: usize) -> Vec<bool> {
        if size > 63 {
            panic!("size must be less than or equal to 63, we got {}", size);
        }
        let mut bits = vec![false; size];
        let mut n = self.n;
        for i in (0..size).rev() {
            bits[i] = n & 1 == 1;
            n >>= 1;
        }
        bits
    }

    pub fn cover<T: Clone>(&self, target: &Vec<T>) -> Vec<T> {
        if target.len() > 63 {
            panic!("size must be less than or equal to 63, we got {}", target.len());
        }
        let bits = self.bits(target.len());
        target.iter().enumerate().filter(|(i, _)| bits[*i]).map(|(_, x)| x.clone()).collect()
    }
}

pub struct BitSearcher {
    n: usize
}

impl BitSearcher {
    pub fn new() -> Self {
        Self { n: 0 }
    }
}

impl Iterator for BitSearcher {
    type Item = BitSearcherResult;

    fn next(&mut self) -> Option<Self::Item> {
        let result = BitSearcherResult::new(self.n);
        self.n += 1;
        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] 
    fn test_result() {
        let target = vec![1, 2, 3, 4, 5];
        let result = BitSearcherResult::new(0);
        assert_eq!(result.bits(3), vec![false, false, false]);
        assert!(result.cover(&target).is_empty());
        let result = BitSearcherResult::new(1);
        assert_eq!(result.bits(3), vec![false, false, true]);
        assert_eq!(result.cover(&target), vec![5]);
        let result = BitSearcherResult::new(2);
        assert_eq!(result.bits(3), vec![false, true, false]);
        assert_eq!(result.cover(&target), vec![4]);
        let result = BitSearcherResult::new((1 << 3) - 1);
        assert_eq!(result.bits(3), vec![true, true, true]);
        assert_eq!(result.bits(4), vec![false, true, true, true]);
        assert_eq!(result.cover(&target), vec![3, 4, 5]);
    }

    #[test]
    fn test_searcher() {
        let mut searcher = BitSearcher::new();
        assert_eq!(searcher.next().unwrap().n, 0);
        assert_eq!(searcher.next().unwrap().n, 1);
        assert_eq!(searcher.next().unwrap().n, 2);
        assert_eq!(searcher.next().unwrap().n, 3);
        
    }
}