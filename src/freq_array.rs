/// Store the frequency of each position in a array.
/// Compute the cumulative frequency by summing over the array.
/// The total is maintained as a separate value.
#[derive(Debug, Clone)]
pub struct FreqTable {
    freqs: Box<[usize]>,
    total: usize,
}

impl super::CumulFreqTable for FreqTable {
    fn new(len: usize) -> Self {
        assert!(len > 0, "table must be non-empty");
        Self {
            freqs: vec![0; len].into_boxed_slice(),
            total: 0,
        }
    }

    fn len(&self) -> usize {
        self.freqs.len()
    }

    fn add(&mut self, pos: usize, val: usize) {
        assert!(pos < self.freqs.len(), "pos out of bounds");
        self.freqs[pos] += val;
        self.total += val;
    }

    fn cumfreq(&self, pos: usize) -> usize {
        assert!(pos < self.freqs.len(), "pos out of bounds");
        self.freqs[..=pos].iter().sum()
    }

    fn total(&self) -> usize {
        self.total
    }

    fn freq(&self, pos: usize) -> usize {
        assert!(pos < self.freqs.len(), "pos out of bounds");
        self.freqs[pos]
    }

    fn find_by_cumfreq(&self, cumfreq: usize) -> usize {
        let mut sum = 0;
        let r = self.freqs.iter().position(|&freq| {
            sum += freq;
            sum >= cumfreq
        });
        // SAFETY: self.freqs is non-empty, so r is always Some.
        unsafe { r.unwrap_unchecked() }
    }

    fn scale_div(&mut self, div_factor: usize) {
        let mut sum = 0;
        for freq in self.freqs.iter_mut() {
            *freq /= div_factor;
            sum += *freq;
        }
        self.total = sum;
    }
}
