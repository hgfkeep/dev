use crate::Solution;

impl Solution {
    /// 对于字符串的最长无重复字符子串，
    ///
    ///       end                              end                             end                             end
    ///        +   max=1                        +   max=2                       +   max=3                       +   max=1
    ///        |                                |                               |                               |
    ///        |                                |                               |                         +-----+
    ///        v                                v                               v                         |     v
    ///   +-----------------+            +-----------------+           +-----------------+           +----v------------+
    ///   |.|.|a|b|c|a|b|.|.|            |.|.|a|b|c|a|b|.|.|           |.|.|a|b|c|a|b|.|.|           |.|.|a|b|c|a|b|.|.|
    ///   +-----------------+            +-----------------+           +-----------------+           +-----------------+
    ///        ^                              ^                             ^                                   ^
    ///        |                              |                             |                                   |
    ///        |                              |                             |                                   |
    ///        +                              +                             +                                   +
    ///      start                          start                         start                               start
    ///
    ///   start = 0                       start = 0                       start = 0                       start = max(0, 1) = 1
    ///   max = 1                         max = 2                         max = 3                         max = max(3, 3) = 3
    ///   index[a] = 1                    index[b] = 2                    index[c] = 3                    index[a] = 4
    ///
    /// 第四个图中，由于`a` 与之前的`a`重复，导致start 需要更新，否则start 不变，仅更新max；
    ///
    /// 下面的解法中，使用index[字符c对应ascii码] 表示当出现重复字符c时，新start的数组下标
    /// 遍历字符串时，比较start 和 index[字符对应ascii码] ， 得到最新的start 位置。（最新的字符位置仅可能为当前字符或者）
    ///
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut max: i32 = 0;
        let mut index: Vec<usize> = vec![0_usize;128]; // 存储字符x的在s中的最新的index，从1开始。
        let mut start: usize = 0;
        for (end, c) in s.chars().enumerate(){
            start = start.max(index[c as usize]);  // 更新start, start 总是重复的字符的下标+1；
            max = max.max((end-start+1) as i32);   // 更新长度
            index[c as usize] = end+1;                   // 更新index
        }
        max
    }
}