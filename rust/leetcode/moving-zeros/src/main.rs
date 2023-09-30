fn move_zeroes(nums: &mut Vec<i32>) {
    let mut no_zeros = nums
        .iter()
        .map(::std::borrow::ToOwned::to_owned)
        .filter(|n| n != &0)
        .collect::<Vec<i32>>();
    no_zeros.resize(nums.len(), 0);
    *nums = no_zeros;
}

fn main() {
    let mut zeros = vec![0, 1, 0, 3 ,12];
    move_zeroes(&mut zeros);
    assert_eq!(zeros, vec![1, 3, 12, 0, 0]);
}
