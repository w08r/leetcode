// Given an integer array nums, rotate the array to the right by k
// steps, where k is non-negative.

#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let ku = nums.len() - (k as usize % nums.len());
        let head = &nums[0..ku];
        let tail = &nums[ku..];
        let onums = [tail, head].concat();
        nums.copy_from_slice(&onums);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_test(nums: &[i32], k: i32, ex_out: &[i32]) {
        let mut n = Vec::from(nums);
        Solution::rotate(&mut n, k);
        assert_eq!(ex_out, n);
    }

    #[test]
    fn test_lc_1() {
        run_test(&[1,2,3,4,5,6,7], 3, &[5,6,7,1,2,3,4]);
    }

    #[test]
    fn test_lc_2() {
        run_test(&[-1,-100,3,99], 2, &[3,99,-1,-100]);
    }

    #[test]
    fn test_lc_3() {
        run_test(&[1,2,3], 4, &[3,1,2]);
    }
}
