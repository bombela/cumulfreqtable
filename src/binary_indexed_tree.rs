use std::ops::{AddAssign, Shl, Sub, SubAssign};

/// store the cumulative frequencies with a binary indexed tree in an array.
/// just as an integer is the sum of appropriate powers of two, so can a cumulative frequency be
/// represented as the appropriate sum of sets of cumulative sub-frequencies.
/// from peter m. fenwick, "a new data structure for cumulative frequency tables." (1994)
///
///
/// The most important operations are O(㏒₂ len) (instead of O(len) for [crate::FreqTable]).
///
/// It is slightly slower than [crate::FreqTable] for small tables depending on the computer. See
/// the [module][crate#benchmarks] documentation for more details.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CumulFreqTable<F = usize> {
    tree: Box<[F]>,
}

impl<F> super::CumulFreqTable<F> for CumulFreqTable<F>
where
    F: std::convert::From<u8>
        + Copy
        + AddAssign
        + SubAssign
        + Sub<Output = F>
        + Shl<u32, Output = F>
        + PartialOrd,
{
    /// Panics if len < 1.
    fn new(len: usize) -> Self {
        assert!(len > 0, "table must be non-empty");
        Self {
            tree: vec![0.into(); len].into_boxed_slice(),
        }
    }

    /// Panics if len < 1.
    /// O(len).
    fn with_freq(len: usize, init: F) -> Self
    where
        usize: TryInto<F>,
        <usize as TryInto<F>>::Error: std::fmt::Debug,
    {
        assert!(len > 0, "table must be non-empty");
        Self {
            tree: (0..len)
                .map(|i| {
                    if i == 0 {
                        init
                    } else {
                        init << i.trailing_zeros()
                    }
                })
                .collect::<Vec<F>>()
                .into_boxed_slice(),
        }
    }

    /// O(1).
    fn len(&self) -> usize {
        self.tree.len()
    }

    /// Panics if pos is out of bounds.
    /// Panics on overflow in debug.
    /// O(㏒₂ len).
    fn add(&mut self, mut pos: usize, val: F) {
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
    /// Panics on underflow in debug.
    /// O(㏒₂ len).
    fn sub(&mut self, mut pos: usize, val: F) {
        assert!(pos < self.tree.len(), "pos out of bounds");
        if pos == 0 {
            self.tree[0] -= val;
        } else {
            while pos < self.tree.len() {
                self.tree[pos] -= val;
                // Add least significant bit.
                // Equivalent to pos += pos & -pos with two's complement.
                pos += 1 << pos.trailing_zeros();
            }
        }
    }

    /// Panics if pos is out of bounds.
    /// O(㏒₂ len).
    fn sum(&self, mut pos: usize) -> F {
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
    fn total(&self) -> F {
        self.sum(self.len() - 1)
    }

    /// O(㏒₂ len).
    fn freq(&self, mut pos: usize) -> F {
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
    fn find_by_sum(&self, mut sum: F) -> usize {
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
    /// scale_freq is called O(len) times (once per position).
    fn scale<C: Fn(F) -> F>(&mut self, scale_freq: C) {
        for pos in (1..self.tree.len()).rev() {
            let freq = self.freq(pos);
            let sfreq = scale_freq(freq);
            if sfreq < freq {
                self.sub(pos, freq - sfreq);
            } else if sfreq > freq {
                self.add(pos, sfreq - freq);
            }
        }
        self.tree[0] = scale_freq(self.tree[0]);
    }
}
