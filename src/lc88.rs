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

use std::{cell::RefCell, rc::Rc};

#[allow(dead_code)]
pub struct Solution {}

// The reverse iterator allows to walk backwards
// through a vector until it hits the beginning
// at which point `has_next` will be false.
//
// Understands the semantics of the vectors used in this leet such
// that it can start from the middle of the first vector (at `m`) and
// the end of the second vecotr.  Allows a reverse output iterator to
// be used to write the current highest value.

struct ReverseIterator<'a> {
    nums: Rc<RefCell<& 'a mut Vec<i32>>>,
    complete: bool,
    e: usize,
}

impl <'a> Iterator for ReverseIterator<'a> {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.complete {
            None
        } else {
            let rv = self.nums.borrow()[self.e];
            if self.e == 0 {
                self.complete = true;
            } else {
                self.e -= 1;
            }
            Some(rv)
        }
    }
}

impl <'a> ReverseIterator<'a> {
    pub fn new(v: &Rc<RefCell<& 'a mut Vec<i32>>>, l: i32) -> ReverseIterator<'a> {
        let len = l as usize;
        if len > 0 {
            ReverseIterator {
                nums: v.clone(),
                complete: false,
                e: len - 1, // index into the vector
            }
        } else {
            // empty
            ReverseIterator {
                nums: v.clone(),
                complete: true,
                e: 0,
            }
        }
    }

    pub fn set(&mut self, val: i32) {
        self.nums.borrow_mut()[self.e] = val
    }
}

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let r1 = Rc::new(RefCell::new(nums1));
        let r2 = Rc::new(RefCell::new(nums2));

        // 2 iterators walk backwards reading from nums1 and nums2
        // whilst the output iterator walks backwards writing to nums1
        let mut n1_iter = ReverseIterator::new(&r1, m);
        let mut n2_iter = ReverseIterator::new(&r2, n);
        // start output at the end, m+n
        let mut out_iter = ReverseIterator::new(&r1, m+n);

        let mut n1 = n1_iter.next();
        let mut n2 = n2_iter.next();
        loop {
            match (n1, n2) {
                (Some(a), Some(b)) if a >= b => {
                    out_iter.set(a);
                    n1 = n1_iter.next();
                }
                (Some(a), None) => {
                    out_iter.set(a);
                    n1 = n1_iter.next();
                }
                (_, Some(b)) => {
                    out_iter.set(b);
                    n2 = n2_iter.next();
                }
                _ => break
            }
            out_iter.next();
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
