use crate::Solution;

impl Solution {
    pub fn new21_game(n: i32, k: i32, w: i32) -> f64 {
        if (k == 0) {
            return 1.0;
        }

        // dp[x] 为 得分为x时， 分数不操作N的概率， 那么 dp[0]即为所求。
        // k <= x <= min(N, k+w-1)时，dp[x]=1 , 其中 x <= N 才能获胜；x<= k+w-1时， 任意获得随机数（范围[1,w]）也能获胜，所以这两个条件得取交集。
        // 当 x > min(N, k+w-1)， dp[x] = 0， 即要么x > N，输了； 要么不可能出现，因为x的最大值为 K +w -1，即前次正好为k-1，且最后一次抽中了W。

        let mut dp: Vec<f64> = vec![0.0_f64; (k + w) as usize];

        let min = n.min(k + w - 1) as usize;
        let w_u = w as usize;
        let w_f = w as f64;

        for i in (k as usize)..=min {
            dp[i] = 1.0;
        }

        // 仅可能是k <= x <= min(N, k+w-1)时，dp[x]=1 ，所以x = k-1时，胜率为 [min(N, k+w-1) -k + 1]/w , 其中分子中的+1是x的取值范围两头均包含在内。
        dp[(k - 1) as usize] = ((w.min(n - k + 1)) as f64) / w_f;

        // 一定要注意k的取值范围！
        if k - 2 >= 0 {
            for i in (0..=((k - 2) as usize)).rev() {
                dp[i] = dp[i + 1] - (dp[i + w_u + 1] - dp[i + 1]) / w_f;
            }
        }

        dp[0]
    }
}
