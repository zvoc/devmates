// 04/09/2019
//
// Given an encoded string, return its decoded string.
//
// The encoding rule is: k[encoded_string], where the encoded_string inside
// the square brackets is being repeated exactly k times. Note that k is
// guaranteed to be a positive integer.
//
// You may assume that the input string is always valid; No extra white
// spaces, square brackets are well-formed, etc. Furthermore, you may assume
// that the original data does not contain any digits and that digits are only
// for those repeat numbers, k. For example, there won't be input like 3a or
// 2[4].
//
// Example:
//  Input: "3[a]2[bc]"
//  Output: "aaabcbc"
//
//  Input: "2[abc]3[cd]ef"
//  Output: "abcabccdcdcdef"
//
//  Input: "3[a2[c]]"
//  Output: "accaccacc"

use std::str::Chars;

fn decode_segment<'a>(mut iter: Chars<'a>, decoded_string: &mut String) -> Chars<'a> {
    while let Some(c) = iter.next() {
        // Stopping condition
        if c == ']' {
            return iter;
        }

        if c.is_alphabetic() {
            decoded_string.push(c);
            continue;
        }

        // Get number of repetitions
        let mut sum = c.to_digit(10).unwrap();
        while let Some(c) = iter.next() {
            if let Some(digit) = c.to_digit(10) {
                sum = 10 * sum + digit;
            } else {
                // This will skip [
                break;
            }
        }

        // Recursion
        let count = sum;
        let start_point = iter.clone();
        for _ in 0..count {
            iter = decode_segment(start_point.clone(), decoded_string);
        };
    }

    return iter;
}

fn decode_string(encoded_string: &str) -> String {
    let mut decoded_string = String::new();
    let iter = decode_segment(encoded_string.chars(), &mut decoded_string);
    assert_eq!(iter.last(), None);
    return decoded_string;
}

fn main() {
    let encoded_string = "3[a]2[bc]";
    println!("Decoded string: {}", decode_string(&encoded_string));
    let encoded_string = "2[abc]3[cd]ef";
    println!("Decoded string: {}", decode_string(&encoded_string));
    let encoded_string = "3[a2[c]]";
    println!("Decoded string: {}", decode_string(&encoded_string));
    let encoded_string = "a2[b2[c]b]a2[aa]";
    println!("Decoded string: {}", decode_string(&encoded_string));
}
