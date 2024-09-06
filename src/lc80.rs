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
        let mut writer = 0_usize;
        let mut reader = writer;

        if nums.is_empty() {
            return 0;
        }

        let mut cur = nums[writer];
        writer += 1;

        if writer < nums.len() && nums[0] == nums[1] {
            writer += 1;
        }

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

                // if the next number is the same, add it
                if (reader+1) < nums.len() && cur == nums[reader+1] {
                    reader += 1;
                    nums[writer] = nums[reader];
                    cur = nums[writer];
                    writer += 1;
                }
            }
        }
        dbg!(&nums);
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
        run_test(&[1, 1, 1, 2, 2, 3], 5, &[1, 1, 2, 2, 3]);
    }

    #[test]
    fn test_leet_2() {
        run_test(&[0, 0, 1, 1, 1, 1, 2, 3, 3], 7, &[0, 0, 1, 1, 2, 3, 3]);
    }
}
