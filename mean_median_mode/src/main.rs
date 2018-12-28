/*
 * Calculate Mean, Median and Mode.
 *
 * Here concepts like reading from stdin, parsing them to integer, vector, hashmap, Option,
 * match, loops are used.
 */

use std::{
    io,
    collections::HashMap
};

fn read_u32(prompt: &str) -> u32 {
    let mut str_buffer = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut str_buffer).expect("Failed to read line");
    let num: u32 = str_buffer.trim().parse().expect("Please type a number!");

    return num;
}

fn read_i32(prompt: &str) -> i32 {
    let mut str_buffer = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut str_buffer).expect("Failed to read line");
    let num: i32 = str_buffer.trim().parse().expect("Please type a number!");

    return num;
}

fn get_n_numbers(count: u32) -> Vec<i32> {
    let mut seq_values: Vec<i32> = vec![];
    let mut count = count;

    while count > 0 {
        seq_values.push(read_i32("Enter a number"));
        count -= 1;
    }

    seq_values
}

fn cal_mean_median_mode(seq_values: &mut Vec<i32>) -> (f32, f32, i32) {
    let mut mean: f32 = 0.0;
    let median;
    let mut mode = 0; // this value doesn't matter
    let mut mode_map = HashMap::new();
    let mut max : Option<i32> = None;

    // mean and mode calculation.
    for &v in seq_values.iter() {
        mean += v as f32;
        let entry_count = mode_map.entry(v).or_insert(0);
        *entry_count += 1;

        max = match max {
            Some(val) => if *entry_count > val {
                mode = v;
                Some(*entry_count)
            } else {
                Some(val)
            },

            None => {
                mode = v;
                Some(*entry_count)
            }
        }
    }
    mean /= seq_values.len() as f32;

    // median calculation
    seq_values.sort();
    let mid = seq_values.len() / 2;

    median = match seq_values.len() % 2 {
        0 => (seq_values[mid] + seq_values[mid-1]) as f32 / 2 as f32,
        _ => seq_values[mid] as f32
    };

    (mean, median, mode)
}

fn main() {
    let count = read_u32("Enter the count");
    let mut seq_values = get_n_numbers(count);
    let (mean, median, mode) = cal_mean_median_mode(&mut seq_values);

    println!("Mean: {}, Median: {}, Mode: {}", mean, median, mode);
}