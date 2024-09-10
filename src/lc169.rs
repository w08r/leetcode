// Given an array nums of size n, return the majority element.

// The majority element is the element that appears more than ⌊n / 2⌋
// times. You may assume that the majority element always exists in
// the array.
#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    // boyer-moore voting
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        return nums.iter().fold(
            (nums[0], 0),
            |acc, i| match acc {
                _ if acc.1 == 0 => (*i, 1),
                x if x.0 == *i  => (*i, acc.1 + 1),
                _               => (acc.0, acc.1 - 1)
            }).0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_test(n: &[i32], ex: i32) {
        let v = Vec::from(n);
        let e = Solution::majority_element(v);
        assert_eq!(e, ex);
    }

    #[test]
    fn test_lc_1() {
        run_test(&[3,2,3], 3);
    }

    #[test]
    fn test_lc_2() {
        run_test(&[2,2,1,1,1,2,2], 2);
    }
}
