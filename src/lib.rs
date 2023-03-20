/// A cumulative frequency table computes the cumulative frequency for each position in the table.
/// Different implementations will offer different performance characteristics.
/// TODO: Generic over the type of the frequency.
pub trait CumFreqTable {
    /// Create a new table with the given length.
    /// Panic:
    fn new(len: usize) -> Self;

    /// Get the length of the table.
    fn len(&self) -> usize;

    /// Add to the frequency of the given position.
    fn add(&mut self, pos: usize, val: usize);

    /// Add one to the frequency of the given position.
    fn inc(&mut self, pos: usize) {
        self.add(pos, 1);
    }

    /// Get the cumulative frequency of the given position.
    fn cumfreq(&self, pos: usize) -> usize;

    /// The total cumulative frequency.
    fn total(&self) -> usize;

    /// Get the frequency of the given position.
    fn freq(&self, pos: usize) -> usize;

    /// Find the position with the given cumulative frequency.
    fn find_by_cumfreq(&self, cumfreq: usize) -> usize;

    /// Scale the table by dividing.
    fn scale_div(&mut self, div_factor: usize);
}

pub mod freq_table;
pub mod cumfreq_table;
pub mod binary_indexed_tree;

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Debug;

    #[test]
    fn it_works() {
        test_linear::<freq_table::FreqTable>();
        test_linear::<cumfreq_table::CumFreqTable>();
        test_linear::<binary_indexed_tree::CumFreqTable>();
    }

    fn test_linear<T: CumFreqTable + Debug>() {
        let len = 10;
        let mut table = T::new(len);
        dbg!(len, &table);
        assert_eq!(table.len(), len);
        assert_eq!(table.total(), 0);
        for i in 0..len {
            assert_eq!(table.freq(i), 0);
            assert_eq!(table.cumfreq(i), 0);
        }
        assert_eq!(table.total(), 0);
        for i in 0..len {
            table.inc(i);
            assert_eq!(table.freq(i), 1);
            assert_eq!(table.cumfreq(i), i + 1);
        }
        assert_eq!(table.total(), len);
        for i in 0..len {
            table.add(i, 2);
            assert_eq!(table.freq(i), 3);
            assert_eq!(table.cumfreq(i), (i + 1) * 3);
        }
        assert_eq!(table.total(), len * 3);
        for i in 0..len {
            assert_eq!(table.freq(i), 3);
            assert_eq!(table.cumfreq(i), (i + 1) * 3);
        }
        dbg!(&table);
        table.scale_div(2);
        dbg!(&table);
        assert_eq!(table.total(), len);
        for i in 0..len {
            assert_eq!(table.freq(i), 3 / 2);
            assert_eq!(table.cumfreq(i), i + 1);
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
            assert_eq!(table.cumfreq(i), ((i+2)/2*42) + ((i+1)/2));
        }
    }
}
