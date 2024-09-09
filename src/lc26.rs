// Given an integer array nums sorted in non-decreasing order, remove
// the duplicates in-place such that each unique element appears only
// once. The relative order of the elements should be kept the
// same. Then return the number of unique elements in nums.

// Consider the number of unique elements of nums to be k, to get
// accepted, you need to do the following things:

// - Change the array nums such that the first k elements of nums
//   contain the unique elements in the order they were present in nums
//   initially. The remaining elements of nums are not important as well
//   as the size of nums.
// - Return k.

#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut writer = 0_usize;
        let mut reader = writer;

        if nums.is_empty() {
            return 0;
        }

        let mut cur = nums[writer];
        writer += 1;

        while reader < nums.len() && writer < nums.len() {
            // find the next number
            while reader < nums.len() && nums[reader] == cur {
                reader += 1;
            }
            // if we're not at the end, insert at writer pos
            if reader < nums.len() {
                nums[writer] = nums[reader];
                cur = nums[writer];
                writer += 1;
            }
        }
        nums.resize(writer, 0);
        writer as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_test(n1: &[i32], ex: i32, ex_arr: &[i32]) {
        let mut v1 = Vec::from(n1);
        let n = Solution::remove_duplicates(&mut v1);
        assert_eq!(n, ex);
        assert_eq!(v1, ex_arr);
    }

    #[test]
    fn test_empty() {
        run_test(&[], 0, &[]);
    }

    #[test]
    fn test_one() {
        run_test(&[1], 1, &[1]);
    }

    #[test]
    fn test_leet_1() {
        run_test(&[1, 1, 2], 2, &[1, 2]);
    }

    #[test]
    fn test_leet_2() {
        run_test(&[0, 0, 1, 1, 1, 2, 2, 3, 3, 4], 5, &[0, 1, 2, 3, 4]);
    }
}
