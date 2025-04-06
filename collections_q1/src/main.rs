// Given a list of integers, use a vector and return the median (when sorted, the value in
// the middle position) and mode (the value that occurs most often; a hash map will be
// helpful here) of the list.
use std::collections::HashMap;

const ARRAY_LEN: usize = 10;

fn get_median(array: [i64; ARRAY_LEN]) -> i64 {
    let mut v = array.to_vec();

    v.sort();

    v[v.len() / 2]
}

fn get_mode(array: [i64; ARRAY_LEN]) -> i64 {
    let mut map = HashMap::new();

    for element in array {
        let element_freq_ref = map.entry(element).or_insert(0);
        *element_freq_ref += 1;
    }
    let mut high_freq = i64::MIN;
    let mut target_key = 0;
    for (key, freq) in map {
        if freq > high_freq {
            high_freq = freq;
            target_key = key;
        }
    }
    target_key
}

fn main() {
    let array = [-1, 5, -2, -5, -4, -3, 2, 3, 4, 1];
    // -5 -4 -3 -2 -1  1  2  3  4  5
    //  0  1  2  3  4  5  6  7  8  9
    let median = get_median(array);
    let mode = get_mode(array);
    println!("median: {median}");
    println!("mode: {mode}");
}
