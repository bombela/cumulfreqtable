use std::{
    iter::Sum,
    ops::{AddAssign, Sub, SubAssign},
};

/// Store the cumulative frequencies of each position in a array.
/// The cumulative frequency is computed on update. In practice this is slightly slower than
/// freq_array::FreqTable because of the extra memory writes. It exbibits identical big-O runtime
/// complexity. And is only useful for validating benchmarks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CumulFreqTable<F = usize> {
    sums: Box<[F]>,
}

impl<F> super::CumulFreqTable<F> for CumulFreqTable<F>
where
    F: std::convert::From<u8> + Copy + AddAssign + SubAssign + Sub<Output = F> + Sum + PartialOrd,
{
    /// Panics if len < 1.
    fn new(len: usize) -> Self {
        assert!(len > 0, "table must be non-empty");
        Self {
            sums: vec![0.into(); len].into_boxed_slice(),
        }
    }

    /// Panics if len < 1.
    fn with_freq(len: usize, init: F) -> Self {
        assert!(len > 0, "table must be non-empty");
        let mut sums = vec![0.into(); len].into_boxed_slice();
        let mut total = init;
        for sum in sums.iter_mut() {
            *sum = total;
            total += init;
        }
        Self { sums }
    }

    // O(1).
    fn len(&self) -> usize {
        self.sums.len()
    }

    // Panics if pos is out of bounds.
    // Panics on overflow in debug.
    // O(len).
    fn add(&mut self, pos: usize, val: F) {
        assert!(pos < self.sums.len(), "pos out of bounds");
        for sum in self.sums[pos..].iter_mut() {
            *sum += val;
        }
    }
    //
    // Panics if pos is out of bounds.
    // Panics on underflow in debug.
    // O(len).
    fn sub(&mut self, pos: usize, val: F) {
        assert!(pos < self.sums.len(), "pos out of bounds");
        for sum in self.sums[pos..].iter_mut() {
            *sum -= val;
        }
    }

    // Panics if pos is out of bounds.
    // O(1).
    fn sum(&self, pos: usize) -> F {
        assert!(pos < self.sums.len(), "pos out of bounds");
        self.sums[pos]
    }

    // O(1).
    fn total(&self) -> F {
        let r = self.sums.last().copied();
        // SAFETY: self.sums is non-empty, so r is always Some.
        unsafe { r.unwrap_unchecked() }
    }

    // Panics if pos is out of bounds.
    // O(1).
    fn freq(&self, pos: usize) -> F {
        assert!(pos < self.sums.len(), "pos out of bounds");
        if pos == 0 {
            self.sums[0]
        } else {
            self.sums[pos] - self.sums[pos - 1]
        }
    }

    // O(len).
    fn find_by_sum(&self, sum: F) -> usize {
        let r = self.sums.iter().position(|&i_sum| i_sum >= sum);
        // SAFETY: self.sums is non-empty, so r is always Some.
        unsafe { r.unwrap_unchecked() }
    }

    // O(len).
    fn scale<C: Fn(F) -> F>(&mut self, scale_freq: C) {
        let mut psum: F = 0.into();
        let mut spsum: F = 0.into();
        for sum in self.sums.iter_mut() {
            spsum += scale_freq(*sum - psum);
            psum = std::mem::replace(sum, spsum);
        }
    }
}
