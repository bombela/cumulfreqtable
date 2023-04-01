/// A cumulative frequency table computes the cumulative frequency for each position in the table.
/// Different implementations will offer different performance characteristics.
/// TODO: Generic over the type of the frequency.
pub trait CumulFreqTable {
    /// Create a new table with the given length.
    fn new(len: usize) -> Self;

    /// Get the length of the table.
    fn len(&self) -> usize;

    /// Add to the frequency of the given position.
    fn add(&mut self, pos: usize, val: usize);

    /// Add one to the frequency of the given position.
    /// A shortcut for `add(pos, 1)`.
    fn inc(&mut self, pos: usize) {
        self.add(pos, 1);
    }

    /// Get the cumulative frequency of the given position.
    fn sum(&self, pos: usize) -> usize;

    /// The total cumulative frequency.
    /// This is the same as the cumulative frequency of the last position, but may be more
    /// efficient depending on the implementation.
    fn total(&self) -> usize;

    /// Get the frequency of the given position.
    fn freq(&self, pos: usize) -> usize;

    /// Find the first position with an equal or greater cumulative frequency.
    fn find_by_sum(&self, sum: usize) -> usize;

    /// Divide every positions by the given factor.
    fn scale_down(&mut self, factor: usize);
}

pub mod freq_array;
pub mod cumulfreq_array;
pub mod binary_indexed_tree;

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Debug;

    #[test]
    fn test_linear() {
        for len in 1..=32 {
            dbg!("freq_array", len);
            test_linear_impl::<freq_array::FreqTable>(len);
            dbg!("cumulfreq_array", len);
            test_linear_impl::<cumulfreq_array::CumulFreqTable>(len);
            dbg!("binary_indexed_tree", len);
            test_linear_impl::<binary_indexed_tree::CumulFreqTable>(len);
        }
    }

    fn test_linear_impl<T: CumulFreqTable + Debug + 'static>(len: usize) {
        let mut table = T::new(len);
        dbg!(len, &table);
        assert_eq!(table.len(), len);
        assert_eq!(table.total(), 0);
        for i in 0..len {
            assert_eq!(table.freq(i), 0);
            assert_eq!(table.sum(i), 0);
        }
        assert_eq!(table.total(), 0);
        for i in 0..len {
            table.inc(i);
            assert_eq!(table.freq(i), 1);
            assert_eq!(table.sum(i), i + 1);
        }
        assert_eq!(table.total(), len);
        for i in 0..len {
            table.add(i, 2);
            assert_eq!(table.freq(i), 3);
            assert_eq!(table.sum(i), (i + 1) * 3);
        }
        assert_eq!(table.total(), len * 3);
        for i in 0..len {
            assert_eq!(table.freq(i), 3);
            assert_eq!(table.sum(i), (i + 1) * 3);
        }
        dbg!(&table);
        for i in 0..len {
            dbg!(i, (i+1)*3, table.sum(i));
            assert_eq!(table.find_by_sum((i + 1) * 3), i);
        }
        dbg!(&table);
        table.scale_down(2);
        dbg!(&table);
        assert_eq!(table.total(), len);
        for i in 0..len {
            assert_eq!(table.freq(i), 3 / 2);
            assert_eq!(table.sum(i), i + 1);
        }
        for i in (0..len).step_by(2) {
            table.add(i, 41);
        }
        assert_eq!(table.total(), (len-len/2)*42+(len/2));
        for i in 0..len {
            if i % 2 == 0 {
                assert_eq!(table.freq(i), 42);
            } else {
                assert_eq!(table.freq(i), 1);
            }
            assert_eq!(table.sum(i), ((i+2)/2*42) + ((i+1)/2));
        }
    }
}
