// You are given an array prices where prices[i] is the price of a
// given stock on the ith day.

// You want to maximize your profit by choosing a single day to buy
// one stock and choosing a different day in the future to sell that
// stock.

// Return the maximum profit you can achieve from this transaction. If
// you cannot achieve any profit, return 0.

use std::cmp;


#[allow(dead_code)]
pub struct Solution {}

fn _diff(t: (i32, i32)) -> i32 {
    t.1 - t.0
}

fn _max_profit(prices: &[i32]) -> (i32,i32,i32,i32) {
    return match prices.len() {
        0 => panic!("logic error"),
        1 => (prices[0],prices[0],prices[0],prices[0]),
        2 if prices[0] < prices[1] => (prices[0],prices[1],prices[0],prices[1]),
        2 => (prices[0],prices[1],prices[1],prices[0]),
        _ => {
            let lhs = _max_profit(&prices[0..prices.len()/2]);
            let rhs = _max_profit(&prices[prices.len()/2..]);
            let mut options = vec![
                (lhs.0,lhs.1),
                (lhs.0,rhs.0),
                (lhs.0,rhs.1),
                (lhs.1,rhs.0),
                (lhs.1,rhs.1),
                (rhs.0,rhs.1),
                (lhs.2,rhs.3)
            ];
            options.sort_by(|b, a| (a.1-a.0).cmp(&(b.1-b.0)));
            (options[0].0,options[0].1,cmp::min(lhs.2,rhs.2),cmp::max(lhs.3,rhs.3))
        }
    }
}

impl Solution {

    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (min, max,_, _) = _max_profit(&prices);
        cmp::max(max - min, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_test(n: &[i32], ex: i32) {
        let v = Vec::from(n);
        let e = Solution::max_profit(v);
        assert_eq!(e, ex);
    }

    #[test]
    fn test_lc_1() {
        run_test(&[7,1,5,3,6,4], 5);
    }

    #[test]
    fn test_lc_2() {
        run_test(&[7,6,4,3,1], 0);
    }

    #[test]
    fn test_lc_3() {
        run_test(&[1,2,4], 3);
    }

    #[test]
    fn test_lc_4() {
        run_test(&[1,2,11,4,7], 10);
    }
}
