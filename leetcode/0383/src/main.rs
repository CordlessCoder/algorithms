pub struct Solution();
impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut letters = [0u32; 26];
        magazine
            .bytes()
            .for_each(|c| letters[(c - b'a') as usize] += 1);

        ransom_note.bytes().all(|c| {
            let l = letters.get_mut((c - b'a') as usize).unwrap();
            if let Some(n) = l.checked_sub(1) {
                *l = n;
                true
            } else {
                false
            }
        })
    }
}
fn main() {
    println!("Hello, world!");
}
