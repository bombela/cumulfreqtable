/// Store the cumulative frequencies with a binary indexed tree in a array.
/// Just as an integer is the sum of appropriate powers of two, so can a cumulative frequency be
/// represented as the appropriate sum of sets of cumulative sub-frequencies.
/// From Peter m. Fenwick, "A new data structure for cumulative frequency tables." (1994)
///
///
/// The most important operations are O(㏒₂ len) (instead of O(len) for [crate::FreqTable]).
///
/// It is slightly slower than [crate::FreqTable] for small tables depending on the computer. See
/// the [module][crate#benchmarks] documentation for more details.
#[derive(Debug, Clone)]
pub struct CumulFreqTable {
    tree: Box<[usize]>,
}

impl super::CumulFreqTable for CumulFreqTable {
    /// Panics if len < 1.
    fn new(len: usize) -> Self {
        assert!(len > 0, "table must be non-empty");
        Self {
            tree: vec![0; len].into_boxed_slice(),
        }
    }

    /// O(1).
    fn len(&self) -> usize {
        self.tree.len()
    }

    /// Panics if pos is out of bounds.
    /// O(㏒₂ len).
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

    /// Panics if pos is out of bounds.
    /// O(㏒₂ len).
    fn sum(&self, mut pos: usize) -> usize {
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

    /// O(㏒₂ len).
    fn total(&self) -> usize {
        self.sum(self.len() - 1)
    }

    /// O(㏒₂ len).
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

    /// O(㏒₂ len).
    fn find_by_sum(&self, mut sum: usize) -> usize {
        // Modified binary search.
        let mut pos = 0;
        if sum > self.tree[0] {
            sum -= self.tree[0];
            // .len() is always >= 1.
            // The -1 and +1 dance is to avoid overflow.
            let mut mid = (((self.len() - 1) / 2) + 1).next_power_of_two();
            /* It is a more efficient version of this:
            let mut mid = self
                .len()
                .checked_next_power_of_two()
                .map(|x| x / 2)
                .unwrap_or(1 << (usize::BITS-1));
            */
            while mid != 0 {
                let hi = pos + mid;
                if hi < self.len() && sum >= self.tree[hi] {
                    pos = hi;
                    sum -= self.tree[pos];
                }
                mid /= 2;
            }
        }
        pos
    }

    /// O(len ㏒₂ len).
    fn scale_down(&mut self, factor: usize) {
        for mut pos in (1..self.tree.len()).rev() {
            // Equivalent to: self.add(pos, - self.freq(pos) / div); if it accepted a signed value.
            let freq = self.freq(pos);
            let sub = freq - freq / factor;
            while pos < self.tree.len() {
                self.tree[pos] -= sub;
                pos += 1 << pos.trailing_zeros();
            }
        }
        self.tree[0] /= factor;
    }
}
