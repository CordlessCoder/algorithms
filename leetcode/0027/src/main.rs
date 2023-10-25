pub struct Solution();
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let (mut l, mut r) = (0, nums.len() - 1);
        while l <= r {
            if nums[l] == val {
                nums.swap(l, r);
                r -= 1;
            } else {
                l += 1
            }
        }
        l as i32 + 1
    }
}
fn main() {}
