
/**
 假设有从 1 到 n 的 n 个整数。用这些整数构造一个数组 perm（下标从 1 开始），只要满足下述条件 之一 ，该数组就是一个 优美的排列 ：
    perm[i] 能够被 i 整除
    i 能够被 perm[i] 整除
 给你一个整数 n ，返回可以构造的 优美排列 的 数量 。

 解题思路：状态压缩 + 记忆化搜索
 */
pub fn count_arrangement(n: i32) -> i32 {
    fn dfs(sum: usize, n: i32, cache: &mut Vec<i32>) -> i32 {
        if sum == (1 << n) - 1 {
            return 1;
        }
        if cache[sum] != -1 {
            return cache[sum];
        }
        let index = sum.count_ones() as i32 + 1;
        let mut res = 0;
        for i in 1..=n {
            if (sum >> (i - 1) & 1) == 0 && (index % i == 0 || i % index == 0) {
                res += dfs(sum | 1 << (i - 1), n, cache);
            }
        }
        cache[sum] = res;
        res
    }
    let mut cache = vec![-1; 1 << n];
    dfs(0, n, &mut cache)
}