use std::{
    iter::Sum,
    ops::{AddAssign, DivAssign},
};

/// Store the frequency of each position in a array.
/// Compute the cumulative frequency on demande by summing over the array.
/// The total is maintained as a separate value.
///
/// It is slightly faster than [crate::BinaryIndexedTree] for small tables depending on the
/// computer. See the [module][crate#benchmarks] documentation for more details.
#[derive(Debug, Clone)]
pub struct FreqTable<F = usize> {
    freqs: Box<[F]>,
    total: F,
}

impl<F> super::CumulFreqTable<F> for FreqTable<F>
where
    F: std::convert::From<u8> + Copy + AddAssign + Sum + DivAssign + PartialOrd,
{
    /// Panics if len < 1.
    fn new(len: usize) -> Self {
        assert!(len > 0, "table must be non-empty");
        Self {
            freqs: vec![0.into(); len].into_boxed_slice(),
            total: 0.into(),
        }
    }

    /// O(1).
    fn len(&self) -> usize {
        self.freqs.len()
    }

    /// Panics if pos is out of bounds.
    /// O(1).
    fn add(&mut self, pos: usize, val: F) {
        assert!(pos < self.freqs.len(), "pos out of bounds");
        self.freqs[pos] += val;
        self.total += val;
    }

    /// Panics if pos is out of bounds.
    /// O(len).
    fn sum(&self, pos: usize) -> F {
        assert!(pos < self.freqs.len(), "pos out of bounds");
        self.freqs[..=pos].iter().copied().sum()
    }

    /// O(1).
    fn total(&self) -> F {
        self.total
    }

    /// Panics if pos is out of bounds.
    /// O(1).
    fn freq(&self, pos: usize) -> F {
        assert!(pos < self.freqs.len(), "pos out of bounds");
        self.freqs[pos]
    }

    /// O(len).
    fn find_by_sum(&self, sum: F) -> usize {
        let mut r_sum: F = 0.into();
        let r = self.freqs.iter().position(|&freq| {
            r_sum += freq;
            r_sum >= sum
        });
        // SAFETY: self.freqs is non-empty, so r is always Some.
        unsafe { r.unwrap_unchecked() }
    }

    /// O(len).
    fn scale_down(&mut self, factor: F) {
        let mut sum: F = 0.into();
        for freq in self.freqs.iter_mut() {
            *freq /= factor;
            sum += *freq;
        }
        self.total = sum;
    }
}
