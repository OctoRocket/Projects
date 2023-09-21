#![deny(
    clippy::all,
    clippy::pedantic,
)]
#![warn(clippy::nursery)]

struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn apply_operations(nums: Vec<i32>) -> Vec<i32> {
        let mut zero_counter = nums.iter().filter(|n| n == &&0).count();
        let mut new_nums = nums;

        for i in 1..new_nums.len() {
            if new_nums[i - 1] == new_nums[i] && new_nums[i] != 0 {
                new_nums[i - 1] *= 2;
                new_nums[i] = 0;
                zero_counter += 1;
            }
        }
        
        new_nums = new_nums.iter().map(::std::borrow::ToOwned::to_owned).filter(|n| n != &0).collect::<Vec<i32>>();
        new_nums.resize(new_nums.len() + zero_counter, 0);

        new_nums
    }
}

fn main() {
    use Solution as S;
    assert_eq!(S::apply_operations(vec![1, 2, 2, 1, 1, 0]), vec![1, 4, 2, 0, 0, 0]);
    assert_eq!(S::apply_operations(vec![0, 1]), vec![1, 0]);
}
