# CumulFreqTable

[![Documentation](https://docs.rs/cumulfreqtable/badge.svg)](https://docs.rs/cumulfreqtable)

Store cumulative frequencies with a binary indexed tree in a array.

Just as an integer is the sum of appropriate powers of two, so can a cumulative frequency be
represented as the appropriate sum of sets of cumulative sub-frequencies.
from peter m. fenwick, "a new data structure for cumulative frequency tables." (1994)

## Definition

A cumulative frequency table stores and/or compute the absolute frequency and the cumulative
frequency for ever position in the table.

Formal definition from [Wikipedia](https://en.wikipedia.org/wiki/Frequency_(statistics)):
> In statistics, the frequency (or absolute frequency) of an event *i* is the number *nᵢ* of
times the observation has occurred/recorded in an experiment or study. The cumulative frequency
is the total of the absolute frequencies of all events at or below a certain point in an
ordered list of events.

## Example

```rust
use cumulfreqtable::CumulFreqTable; // Import the trait in scope.

let mut table = cumulfreqtable::BinaryIndexedTree::new(16);
table.inc(0);
table.inc(3);
table.add(5, 3);

assert_eq!(table.freq(0), 1);
assert_eq!(table.freq(3), 1);
assert_eq!(table.freq(5), 3);
assert_eq!(table.freq(6), 0);

assert_eq!(table.sum(0), 1);
assert_eq!(table.sum(3), 2);
assert_eq!(table.sum(5), 5);
assert_eq!(table.sum(6), 5);

assert_eq!(table.total(), 5);
```

# Implementations

This crate offers two implementations of the [CumulFreqTable] trait:
- [FreqTable]: stores the absolute frequency of every positions in an array O(1)
but computes the cumulative frequency in O(len). Best for small tables.
- [BinaryIndexedTree]: stores the cumulative frequency of every positions in
a binary indexed tree. The runtime complexity is O(㏒₂ len) for all operations.
