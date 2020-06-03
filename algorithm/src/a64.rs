use crate::Solution;

/// 无法使用判断语句，循环语句等
impl Solution {
    ///正常解法
    pub fn sum_nums_nomal(n: i32) -> i32 {
        // rust immutable value n
        let mut x = n;
        if n > 0 {
            x += Solution::sum_nums(n - 1);
        }
        x
    }

    /// rust unstalble 解法
    pub fn sum_nums_unstable(n: i32) -> i32 {
        let mut x = n;
        let _ = (n > 0).then(|| x += Solution::sum_nums(n - 1));
        x
    }

    /// 合理的解法
    pub fn sum_nums(n: i32) -> i32 {
        // rust immutable value n
        let mut x = n;
        let _ = n > 0 && (x += Solution::sum_nums(n - 1)) == ();
        x
    }
}
