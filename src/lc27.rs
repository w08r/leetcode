// Given an integer array nums and an integer val, remove all
// occurrences of val in nums in-place. The order of the elements may
// be changed. Then return the number of elements in nums which are
// not equal to val.

// Consider the number of elements in nums which are not equal to val
// be k, to get accepted, you need to do the following things:

// - Change the array nums such that the first k elements of nums
//   contain the elements which are not equal to val. The remaining
//   elements of nums are not important as well as the size of nums.
// - Return k.

#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        // use 2 pointer apporach to move the non-`vals` to the
        // beginning and then chop off the end
        let mut p1 = 0_usize;
        let mut p2 = 0_usize;
        loop {
            while p1 < nums.len() && nums[p1] != val {
                p1 += 1;
            }
            if p1 == nums.len() {
                break;
            }
            // p1 points to val
            p2 = p1 + 1;
            while p2 < nums.len() && nums[p2] == val {
                p2 += 1;
            }
            if p2 == nums.len() {
                break;
            } else {
                // swap
                let t = nums[p1];
                nums[p1] = nums[p2];
                nums[p2] = t;
            }
        }
        nums.resize(p1, val);
        nums.len().try_into().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_test(nums: &[i32], val: i32, ex: i32, ex_out: &[i32]) {
        let mut n = Vec::from(nums);
        let r = Solution::remove_element(&mut n, val);
        assert_eq!(n, ex_out);
        assert_eq!(r, ex);
    }

    #[test]
    fn test_empty() {
        run_test(&[], 0, 0, &[]);
    }

    #[test]
    fn test_one() {
        run_test(&[1], 1, 0, &[]);
        run_test(&[1], 2, 1, &[1]);
    }

    #[test]
    fn test_leet_1() {
        run_test(&[3,2,2,3], 3, 2, &[2, 2]);
    }

    #[test]
    fn test_leet_2() {
        run_test(&[0,1,2,2,3,0,4,2], 2, 5, &[0,1,3,0,4]);
    }
}
