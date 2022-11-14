impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut answer = 0;
        let mut last_min = prices[0];
        for cur_price in prices {
            answer = std::cmp::max(answer, cur_price - last_min);
            last_min = std::cmp::min(last_min, cur_price);
        }
        answer
    }
}
