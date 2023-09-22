#![deny(
    clippy::all,
    clippy::pedantic,
)]
#![warn(clippy::nursery)]

struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn apply_operations(mut nums: Vec<i32>) -> Vec<i32> {
        let mut zero_counter = nums.iter().filter(|n| n == &&0).count();

        for i in 1..nums.len() {
            if nums[i - 1] == nums[i] && nums[i] != 0 {
                nums[i - 1] *= 2;
                nums[i] = 0;
                zero_counter += 1;
            }
        }
        
        nums = nums.iter().map(|&n| n).filter(|&n| n != 0).collect::<Vec<i32>>();
        nums.resize(nums.len() + zero_counter, 0);

        nums
    }
}

fn main() {
    use Solution as S;
    assert_eq!(S::apply_operations(vec![1, 2, 2, 1, 1, 0]), vec![1, 4, 2, 0, 0, 0]);
    assert_eq!(S::apply_operations(vec![0, 1]), vec![1, 0]);
}
