/// Store the cumulative frequencies of each position in a array.
/// Add to the cumulative frequency of a position by adding to all precedent position.
#[derive(Debug, Clone)]
pub struct CumFreqTable {
    sums: Box<[usize]>,
}

impl super::CumFreqTable for CumFreqTable {
    fn new(len: usize) -> Self {
        assert!(len > 0, "table must be non-empty");
        Self {
            sums: vec![0; len].into_boxed_slice(),
        }
    }

    fn len(&self) -> usize {
        self.sums.len()
    }

    fn add(&mut self, pos: usize, val: usize) {
        for sum in self.sums[pos..].iter_mut() {
            *sum += val;
        }
    }

    fn cumfreq(&self, pos: usize) -> usize {
        self.sums[pos]
    }

    fn total(&self) -> usize {
        self.sums[self.len() - 1]
    }

    fn freq(&self, pos: usize) -> usize {
        if pos == 0 {
            self.sums[0]
        } else {
            self.sums[pos] - self.sums[pos - 1]
        }
    }

    fn find_by_cumfreq(&self, cumfreq: usize) -> usize {
        let r = self.sums.iter().position(|&sum| sum >= cumfreq);
        // SAFETY: self.freqs is non-empty, so r is always Some.
        unsafe { r.unwrap_unchecked() }
    }

    fn scale_div(&mut self, div_factor: usize) {
        let mut psum = 0;
        let mut spsum = 0;
        for sum in self.sums.iter_mut() {
            spsum += (*sum - psum) / div_factor;
            psum = std::mem::replace(sum, spsum);
        }
    }
}
