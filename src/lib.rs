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
    fn it_works() {
        test_linear::<freq_array::FreqTable>();
        test_linear::<cumulfreq_array::CumulFreqTable>();
        test_linear::<binary_indexed_tree::CumulFreqTable>();
    }

    fn test_linear<T: CumulFreqTable + Debug>() {
        let len = 10;
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
        assert_eq!(table.total(), 5*42+5);
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
