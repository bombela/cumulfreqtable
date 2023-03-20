/// Store the cumulative frequencies with a binary indexed tree in a array.
/// Just as an integer is the sum of appropriate powers of two, so can a cumulative frequency be
/// represented as the appropriate sum of sets of cumulative sub-frequencies.
/// From Peter m. Fenwick, "A new data structure for cumulative frequency tables." (1994)
#[derive(Debug, Clone)]
pub struct CumulFreqTable {
    tree: Box<[usize]>,
}

impl super::CumFreqTable for CumulFreqTable {
    fn new(len: usize) -> Self {
        assert!(len > 0, "table must be non-empty");
        Self {
            tree: vec![0; len].into_boxed_slice(),
        }
    }

    fn len(&self) -> usize {
        self.tree.len()
    }

    fn add(&mut self, mut pos: usize, val: usize) {
        assert!(pos < self.tree.len(), "pos out of bounds");
        if pos == 0 {
            self.tree[0] += val;
        } else {
            while pos < self.tree.len() {
                self.tree[pos] += val;
                // Add least significant bit.
                // Equivalent to pos += pos & -pos with two's complement.
                pos += 1 << pos.trailing_zeros();
            }
        }
    }

    fn cumfreq(&self, mut pos: usize) -> usize {
        assert!(pos < self.tree.len(), "pos out of bounds");
        let mut sum = self.tree[0];
        while pos > 0 {
            sum += self.tree[pos];
            // Remove least significant bit.
            // Equivalent to pos &= pos - 1;
            pos -= 1 << pos.trailing_zeros();
        }
        sum
    }

    fn total(&self) -> usize {
        self.cumfreq(self.len() - 1)
    }

    fn freq(&self, mut pos: usize) -> usize {
        assert!(pos < self.tree.len(), "pos out of bounds");
        let mut freq = self.tree[pos];
        if pos > 0 {
            let parent = pos - (1 << pos.trailing_zeros());
            pos -= 1;
            while parent != pos {
                freq -= self.tree[pos];
                pos -= 1 << pos.trailing_zeros();
            }
        }
        freq
    }

    fn find_by_cumfreq(&self, mut cumfreq: usize) -> usize {
        // Modified binary search.
        let mut pos = 0;
        if cumfreq > self.tree[0] {
            let mut mid = self.tree.len() / 2;
            while mid != 0 {
                let hi = pos + mid;
                if cumfreq >= self.tree[hi] {
                    pos = hi;
                    cumfreq -= self.tree[pos];
                }
                mid /= 2;
            }
        }
        pos
    }

    fn scale_div(&mut self, div_factor: usize) {
        for mut pos in (1..self.tree.len()).rev() {
            // Equivalent to: self.add(pos, - self.freq(pos) / div); if it accepted a signed value.
            let freq = self.freq(pos);
            let sub = freq - freq / div_factor;
            while pos < self.tree.len() {
                self.tree[pos] -= sub;
                pos += 1 << pos.trailing_zeros();
            }
        }
        self.tree[0] /= div_factor;
    }
}
