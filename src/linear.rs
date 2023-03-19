/// Linear because computing the cumulative frequency of a position requires to iterate the backing
/// array up to the position.
#[derive(Debug, Clone)]
pub struct CumFreqTableLinear {
    freqs: Box<[usize]>,
    total: usize,
}

impl super::CumFreqTable for CumFreqTableLinear {
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
        self.freqs[pos] += val;
        self.total += val;
    }

    fn cumfreq(&self, pos: usize) -> usize {
        self.freqs[..=pos].iter().sum()
    }

    fn total(&self) -> usize {
        self.total
    }

    fn freq(&self, pos: usize) -> usize {
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

    fn scale_div(&mut self, div: usize) {
        let mut sum = 0;
        for freq in self.freqs.iter_mut() {
            *freq /= div;
            sum += *freq;
        }
        self.total = sum;
    }
}
