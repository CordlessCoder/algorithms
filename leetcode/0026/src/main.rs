pub struct Solution();
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut write = 0;
        for read in 1..nums.len() {
            if nums[read] == nums[write] {
            } else {
                write += 1;
                nums.swap(read, write);
            }
        }
        write as i32 + 1
    }
}
fn main() {
    println!("{:?}", {
        let mut v = vec![1, 1, 2, 3];
        let i = Solution::remove_duplicates(&mut v);
        (i, v)
    });
}
