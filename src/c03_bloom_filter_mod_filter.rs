

struct BloomFilter {
    /// Max number of factors
    max_factors: usize,

    /// False positive rate
    false_positive_rate: f64,

    m: usize,

    k: usize,

    bit_array: Vec<bool>,
}