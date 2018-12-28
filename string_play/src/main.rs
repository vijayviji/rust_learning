/**
 * Problem: Convert strings to pig latin. The first consonant of each word is moved to the end of
 * the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have
 * “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about
 * UTF-8 encoding!
 */

/**
 * Learnings:
 *  string literals such as "randomaodfas" is actually a referenece to the string "randomaodfas"
 *  located in data portion of the binary.
 *  string slices such as {let s = String::from("adsf"); &s[0..2]} is also reference to part of
 *  the string located in the heap pointed by s.
 *
 *  string references are of type '&str'
 */

use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut input = input.trim().to_string();

    input = match &input[..1] {
        "a" | "e" | "i" | "o" | "u" => {
            input + "-hay"
        },
        _ => {
            format!("{}-{}ay", &input[1..], &input[0..1])
        }
    };

    println!("Result: {}", input)
}
