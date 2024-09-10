// Given an integer array nums sorted in non-decreasing order, remove
// some duplicates in-place such that each unique element appears at
// most twice. The relative order of the elements should be kept the
// same.

// Since it is impossible to change the length of the array in some
// languages, you must instead have the result be placed in the first
// part of the array nums. More formally, if there are k elements
// after removing the duplicates, then the first k elements of nums
// should hold the final result. It does not matter what you leave
// beyond the first k elements.

// Return k after placing the final result in the first k slots of
// nums.

// Do not allocate extra space for another array. You must do this by
// modifying the input array in-place with O(1) extra memory.


#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() <= 1 {
            return nums.len() as i32;
        }

        let mut writer = 1_usize;

        for reader in 2..nums.len() {
            if nums[reader] == nums[writer] &&
                nums[reader] == nums[writer-1] {
            } else {
                writer += 1;
                nums[writer] = nums[reader];
            }
        }
        nums.resize(writer+1, 0);
        nums.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_test(n1: &[i32], ex: i32, ex_arr: &[i32]) {
        let mut v1 = Vec::from(n1);
        let n = Solution::remove_duplicates(&mut v1);
        assert_eq!(v1, ex_arr);
        assert_eq!(n, ex);
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
        run_test(&[1, 1, 1, 2, 2, 3], 5, &[1, 1, 2, 2, 3]);
    }

    #[test]
    fn test_leet_2() {
        run_test(&[0, 0, 1, 1, 1, 1, 2, 3, 3], 7, &[0, 0, 1, 1, 2, 3, 3]);
    }
}
