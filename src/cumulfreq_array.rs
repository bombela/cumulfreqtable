/// Store the cumulative frequencies of each position in a array.
/// The cumulative frequency is computed on update. In practice this is slightly slower than
/// freq_array::FreqTable because of the extra memory writes. It exbibits identical big-O runtime
/// complexity. And is only useful for validating benchmarks.
#[derive(Debug, Clone)]
pub struct CumulFreqTable {
    sums: Box<[usize]>,
}

impl super::CumulFreqTable for CumulFreqTable {
    /// Panics if len < 1.
    fn new(len: usize) -> Self {
        assert!(len > 0, "table must be non-empty");
        Self {
            sums: vec![0; len].into_boxed_slice(),
        }
    }

    // O(1).
    fn len(&self) -> usize {
        self.sums.len()
    }

    // Panics if pos is out of bounds.
    // O(len).
    fn add(&mut self, pos: usize, val: usize) {
        assert!(pos < self.sums.len(), "pos out of bounds");
        for sum in self.sums[pos..].iter_mut() {
            *sum += val;
        }
    }

    // Panics if pos is out of bounds.
    // O(1).
    fn sum(&self, pos: usize) -> usize {
        assert!(pos < self.sums.len(), "pos out of bounds");
        self.sums[pos]
    }

    // O(1).
    fn total(&self) -> usize {
        let r = self.sums.last().copied();
        // SAFETY: self.sums is non-empty, so r is always Some.
        unsafe { r.unwrap_unchecked() }
    }

    // Panics if pos is out of bounds.
    // O(1).
    fn freq(&self, pos: usize) -> usize {
        assert!(pos < self.sums.len(), "pos out of bounds");
        if pos == 0 {
            self.sums[0]
        } else {
            self.sums[pos] - self.sums[pos - 1]
        }
    }

    // O(len).
    fn find_by_sum(&self, sum: usize) -> usize {
        let r = self.sums.iter().position(|&i_sum| i_sum >= sum);
        // SAFETY: self.sums is non-empty, so r is always Some.
        unsafe { r.unwrap_unchecked() }
    }

    // O(len).
    fn scale_down(&mut self, factor: usize) {
        let mut psum = 0;
        let mut spsum = 0;
        for sum in self.sums.iter_mut() {
            spsum += (*sum - psum) / factor;
            psum = std::mem::replace(sum, spsum);
        }
    }
}
