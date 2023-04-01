/// Store the cumulative frequencies of each position in a array.
/// Add to the cumulative frequency of a position by adding to all precedent position.
#[derive(Debug, Clone)]
pub struct CumulFreqTable {
    sums: Box<[usize]>,
}

impl super::CumulFreqTable for CumulFreqTable {
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
        assert!(pos < self.sums.len(), "pos out of bounds");
        for sum in self.sums[pos..].iter_mut() {
            *sum += val;
        }
    }

    fn cumfreq(&self, pos: usize) -> usize {
        assert!(pos < self.sums.len(), "pos out of bounds");
        self.sums[pos]
    }

    fn total(&self) -> usize {
        let r = self.sums.last().copied();
        // SAFETY: self.sums is non-empty, so r is always Some.
        unsafe { r.unwrap_unchecked() }
    }

    fn freq(&self, pos: usize) -> usize {
        assert!(pos < self.sums.len(), "pos out of bounds");
        if pos == 0 {
            self.sums[0]
        } else {
            self.sums[pos] - self.sums[pos - 1]
        }
    }

    fn find_by_cumfreq(&self, cumfreq: usize) -> usize {
        let r = self.sums.iter().position(|&sum| sum >= cumfreq);
        // SAFETY: self.sums is non-empty, so r is always Some.
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
