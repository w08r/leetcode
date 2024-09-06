// You are given two integer arrays nums1 and nums2, sorted in
// non-decreasing order, and two integers m and n, representing the
// number of elements in nums1 and nums2 respectively.
//
// Merge nums1 and nums2 into a single array sorted in non-decreasing order.
//
// The final sorted array should not be returned by the function, but
// instead be stored inside the array nums1. To accommodate this,
// nums1 has a length of m + n, where the first m elements denote the
// elements that should be merged, and the last n elements are set to
// 0 and should be ignored. nums2 has a length of n.

#[allow(dead_code)]
pub struct Solution {}

// The reverse iterator allows to walk backwards
// through a vector until it hits the beginning
// at which point `has_next` will be false.
//
// Understands the semantics of the vectors used in this leet such
// that it will start from the middle of the first vector (at `m`)
// and the end of the second vecotr.

struct ReverseIterator {
    complete: bool,
    e: usize,
}

impl ReverseIterator {
    pub fn new(ilen: i32) -> ReverseIterator {
        let len = ilen as usize;
        if len > 0 {
            ReverseIterator {
                complete: false,
                e: len - 1,
            }
        } else {
            // empty
            ReverseIterator {
                complete: true,
                e: 0,
            }
        }
    }
    pub fn has_next(&self) -> bool {
        !self.complete
    }
    pub fn next(&mut self) -> usize {
        if self.complete {
            panic!("Can't pop from empty vector")
        }
        let rv = self.e;
        if self.e == 0 {
            self.complete = true;
        } else {
            self.e -= 1;
        }
        rv
    }
    pub fn peek(&mut self) -> usize {
        self.e
    }
}

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut n1_iter = ReverseIterator::new(m);
        let mut n2_iter = ReverseIterator::new(n);

        let mut cur = nums1.len();

        while n1_iter.has_next() || n2_iter.has_next() {
            cur -= 1;
            if !n1_iter.has_next() {
                nums1[cur] = nums2[n2_iter.next()];
            } else if !n2_iter.has_next() || nums1[n1_iter.peek()] >= nums2[n2_iter.peek()] {
                nums1[cur] = nums1[n1_iter.next()];
            } else {
                nums1[cur] = nums2[n2_iter.next()];
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_test(n1: &[i32], m: i32, n2: &[i32], ex: &[i32]) {
        let mut v1 = Vec::from(n1);
        let mut v2 = Vec::from(n2);
        let n = v2.len() as i32;
        Solution::merge(&mut v1, m, &mut v2, n);
        assert_eq!(v1, ex);
    }

    #[test]
    fn test_empty() {
        run_test(&[], 0, &[], &[]);
    }

    #[test]
    fn test_one() {
        run_test(&[1, 0], 1, &[2], &[1, 2]);
    }

    #[test]
    fn test_leet_1() {
        run_test(&[1, 2, 3, 0, 0, 0], 3, &[4, 5, 6], &[1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_leet_2() {
        run_test(&[0], 0, &[1], &[1]);
    }
}
