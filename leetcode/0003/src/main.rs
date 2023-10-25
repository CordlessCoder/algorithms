use std::collections::VecDeque;

fn sample(s: String) -> i32 {
    let mut res = 0;
    let mut queue = VecDeque::new();
    for c in s.chars() {
        if let Some(idx) = queue.iter().position(|&d| c == d) {
            queue.drain(..=idx);
        }
        queue.push_back(c);
        res = res.max(queue.len());
    }
    res as i32
}

// 000000000000...
// 000000...1...00000
// 111111111000000
// aaaaaaaaaaaaaaaaaaaaaaaaaaa - abcd - aaaaaaaaaa

type Bits = u128;
#[inline(always)]
fn ascii_to_bitshift(byte: u8) -> Bits {
    0b1 << (byte)
}

pub fn length_of_longest_substring(input: &str) -> i32 {
    let text = input.as_bytes();
    let len = text.len();
    if len == 0 {
        return 0;
    };
    let mut mask = ascii_to_bitshift(text[0]);
    let (mut idx_first, mut idx_last): (usize, usize) = (0, 0);
    let mut longest = 0i32;
    while idx_last < len {
        let window_len: i32 = (idx_last + 1 - idx_first) as i32;
        if mask.count_ones() == window_len as u32 {
            longest = longest.max(window_len);
            idx_last += 1;
            if idx_last < len {
                mask ^= ascii_to_bitshift(text[idx_last])
            } else {
                return longest;
            }
        } else {
            mask ^= ascii_to_bitshift(text[idx_first]);
            idx_first += 1;
        }
    }
    longest
}

fn main() {
    let input = format!("{0}bce!{0}abcdegf{0}", "a".repeat(100000000));
    println!("{}", sample(input))
}
