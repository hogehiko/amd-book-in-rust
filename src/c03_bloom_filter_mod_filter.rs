use std::f64::{ consts::LN_2};

use murmur3;

#[derive(Debug)]
struct BloomFilter {
    /// Max number of factors
    n_max_factors: usize,

    /// False positive rate
    f_false_positive_rate: f64,

    m_bit_array_length: usize,

    k_hash_functions_count: usize,

    bit_array: Vec<bool>,
}

fn optimal_m_bit_array_size(n_max_factors: usize, p_false_positive_rate: f64) -> usize {
    let m = - (p_false_positive_rate as f64).ln() * n_max_factors as f64 / LN_2.powi(2);
    m.ceil() as usize
}

fn optimal_k_hash_functions_count(n_max_factors: usize, m_bit_array_length: usize) -> usize {
    let k = m_bit_array_length as f64 * LN_2 / n_max_factors as f64;
    k.ceil() as usize
}

impl BloomFilter{
    pub fn new(n_max_factors: usize, f_false_positive_rate: f64) -> Self {
        let m_bit_array_length = optimal_m_bit_array_size(n_max_factors, f_false_positive_rate);
        let k_hash_functions_count = optimal_k_hash_functions_count(n_max_factors, m_bit_array_length);

        BloomFilter {
            n_max_factors,
            f_false_positive_rate,
            m_bit_array_length,
            k_hash_functions_count,
            bit_array: vec![false; m_bit_array_length],
        }
    }

    pub fn insert(&mut self, item: &String) {
        for i in 0..self.k_hash_functions_count {
            let mut cursor = std::io::Cursor::new(item.clone());
            let index = murmur3::murmur3_x64_128(&mut cursor, i as u32).unwrap() as usize % self.m_bit_array_length;
            self.bit_array[index] = true;
            dbg!(&self.bit_array);
        }
    }

    pub fn lookup(&self, item: &String) -> bool {
        for i in 0..self.k_hash_functions_count{
            let mut cursor = std::io::Cursor::new(item.clone());
            let index = murmur3::murmur3_x64_128(&mut cursor, i as u32).unwrap() as usize % self.m_bit_array_length;
            if !self.bit_array[index] {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test(){
        let mut bf = BloomFilter::new(10, 0.01);
        dbg!(&bf);
        bf.insert(&"1".to_string());
        bf.insert(&"2".to_string());
        bf.insert(&"42".to_string());

        dbg!(bf.lookup(&"1".to_string()));
        dbg!(bf.lookup(&"2".to_string()));
        dbg!(bf.lookup(&"3".to_string()));
        dbg!(bf.lookup(&"42".to_string()));
        dbg!(bf.lookup(&"43".to_string()));
    }
}