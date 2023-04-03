//! A cumulative frequency table stores and/or compute the absolute frequency and the cumulative
//! frequency for ever position in the table.
//! 
//! Formal definition from [Wikipedia](https://en.wikipedia.org/wiki/Frequency_(statistics)):
//! > In statistics, the frequency (or absolute frequency) of an event *i* is the number *nᵢ* of
//! times the observation has occurred/recorded in an experiment or study. The cumulative frequency
//! is the total of the absolute frequencies of all events at or below a certain point in an
//! ordered list of events.
//!
//! # Example
//!
//! ```rust
//! use cumulfreqtable::CumulFreqTable; // Import the trait in scope.
//!
//! let mut table = cumulfreqtable::BinaryIndexedTree::new(16);
//! table.inc(0);
//! table.inc(3);
//! table.add(5, 3);
//! 
//! assert_eq!(table.freq(0), 1);
//! assert_eq!(table.freq(3), 1);
//! assert_eq!(table.freq(5), 3);
//! assert_eq!(table.freq(6), 0);
//!
//! assert_eq!(table.sum(0), 1);
//! assert_eq!(table.sum(3), 2);
//! assert_eq!(table.sum(5), 5);
//! assert_eq!(table.sum(6), 5);
//!
//! assert_eq!(table.total(), 5);
//! ```
//!
//! # Implementations
//!
//! This crate offers two implementations of the [CumulFreqTable] trait:
//! - [FreqTable]: stores the absolute frequency of every positions in an array O(1)
//! but computes the cumulative frequency in O(len). Best for small tables.
//! - [BinaryIndexedTree]: stores the cumulative frequency of every positions in
//! a binary indexed tree. The runtime complexity is O(㏒₂ len) for all operations.
//!
//! There is also [cumulfreq_array::CumulFreqTable] that computes and stores the cumulative
//! frequency of every positions on update. It's only purpose is to validate the benchmark results.
//!
//! ## Benchmarks
//!
//! For small tables, [FreqTable] is slightly faster than [BinaryIndexedTree], presumably because
//! it fits within the CPU cache. The cross-over point depends on the computer.
//!
//! You can run the benchmarks yourself with `cargo criterion`.
//!
//! Notice that the graphs below have logarithmic scales.
//!
//! #### Intel i7-4790K (2014)
//! On a 2014 Intel i7-4790K with dual-channel DDR3 1600 Mhz, the binary_indexed_tree becomes
//! faster somewhere above 256×64 bits elements.
#![doc = include_str!("bench_intel_2014.md")]
//!
//! #### AMD Ryzen 7 PRO 5850U (2021)
//! On a 2021 AMD Ryzen 7 PRO 5850U with dual-channel DDR4 3200 Mhz, the binary_indexed_tree becomes
//! faster somewhere above 512×64 bits elements.
#![doc = include_str!("bench_amd_2021.md")]

/// A cumulative frequency table maintains the cumulative frequency for every positions in the
/// table. Different implementations offer different performance characteristics.
///
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
    /// This is the same as the cumulative frequency of the last position, but more efficient
    /// depending on the implementation.
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

pub use freq_array::FreqTable;
pub use binary_indexed_tree::CumulFreqTable as BinaryIndexedTree;

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
