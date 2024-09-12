// You are given an array prices where prices[i] is the price of a
// given stock on the ith day.

// You want to maximize your profit by choosing a single day to buy
// one stock and choosing a different day in the future to sell that
// stock.

// Return the maximum profit you can achieve from this transaction. If
// you cannot achieve any profit, return 0.

#[allow(dead_code)]
pub struct Solution {}

fn _max_profit(prices: &[i32]) -> i32 {
    if prices.is_empty() {
        return 0;
    }

    let mut max_profit = 0;
    let mut current_min = prices[0];
    let mut min_price = current_min;

    for p in &prices[1..] {
        if *p < current_min {
            current_min = *p;
        }
        else {
            let d = p - min_price;
            if d > max_profit {
                max_profit = d;
            }
            if p - current_min > max_profit {
                min_price = current_min;
                max_profit = d;
            }
        }
    }
    max_profit
}

impl Solution {

    pub fn max_profit(prices: Vec<i32>) -> i32 {
        _max_profit(&prices)
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
