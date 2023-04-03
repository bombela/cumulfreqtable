/// Store the frequency of each position in a array.
/// Compute the cumulative frequency on demande by summing over the array.
/// The total is maintained as a separate value.
///
/// It is slightly faster than [crate::BinaryIndexedTree] for small tables depending on the
/// computer. See the [module][crate#benchmarks] documentation for more details.
#[derive(Debug, Clone)]
pub struct FreqTable {
    freqs: Box<[usize]>,
    total: usize,
}

impl super::CumulFreqTable for FreqTable {
    /// Panics if len < 1.
    fn new(len: usize) -> Self {
        assert!(len > 0, "table must be non-empty");
        Self {
            freqs: vec![0; len].into_boxed_slice(),
            total: 0,
        }
    }

    /// O(1).
    fn len(&self) -> usize {
        self.freqs.len()
    }

    /// Panics if pos is out of bounds.
    /// O(1).
    fn add(&mut self, pos: usize, val: usize) {
        assert!(pos < self.freqs.len(), "pos out of bounds");
        self.freqs[pos] += val;
        self.total += val;
    }

    /// Panics if pos is out of bounds.
    /// O(len).
    fn sum(&self, pos: usize) -> usize {
        assert!(pos < self.freqs.len(), "pos out of bounds");
        self.freqs[..=pos].iter().sum()
    }

    /// O(1).
    fn total(&self) -> usize {
        self.total
    }

    /// Panics if pos is out of bounds.
    /// O(1).
    fn freq(&self, pos: usize) -> usize {
        assert!(pos < self.freqs.len(), "pos out of bounds");
        self.freqs[pos]
    }

    /// O(len).
    fn find_by_sum(&self, sum: usize) -> usize {
        let mut r_sum = 0;
        let r = self.freqs.iter().position(|&freq| {
            r_sum += freq;
            r_sum >= sum
        });
        // SAFETY: self.freqs is non-empty, so r is always Some.
        unsafe { r.unwrap_unchecked() }
    }

    /// O(len).
    fn scale_down(&mut self, factor: usize) {
        let mut sum = 0;
        for freq in self.freqs.iter_mut() {
            *freq /= factor;
            sum += *freq;
        }
        self.total = sum;
    }
}
