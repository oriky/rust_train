/*
目标和

给定一个正整数数组 nums 和一个整数 target 。

向数组中的每个整数前添加 '+' 或 '-' ，然后串联起所有整数，可以构造一个 表达式 ：

例如，nums = [2, 1] ，可以在 2 之前添加 '+' ，在 1 之前添加 '-' ，然后串联起来得到表达式 "+2-1" 。
返回可以通过上述方法构造的、运算结果等于 target 的不同 表达式 的数目。

示例 1：

输入：nums = [1,1,1,1,1], target = 3
输出：5
解释：一共有 5 种方法让最终目标和为 3 。
-1 + 1 + 1 + 1 + 1 = 3
+1 - 1 + 1 + 1 + 1 = 3
+1 + 1 - 1 + 1 + 1 = 3
+1 + 1 + 1 - 1 + 1 = 3
+1 + 1 + 1 + 1 - 1 = 3

    1 <= nums.length <= 20
    0 <= nums[i] <= 1000
    0 <= sum(nums[i]) <= 1000
    -1000 <= target <= 1000
*/
pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
    // 令nums的所有元素的和为s, 假设选择了nums数组中x个数的负数使得sum(nums) = target，其中x个数的和为n（n是一个正数）
    // 则正数和p为：p = s - n
    // 如果和恰好为target，则：target = 正数和 - 负数和 = s - n - n = s - 2n
    // 对于n有如下约束：n = (s - target) / 2
    // 可以得到两个条件：n > 0, n % 2 = 0(偶数)
    let sum: i32 = nums.iter().sum();
    let diff = sum - target;
    if (diff < 0 || diff % 2 == 1) {
        return 0;
    }
    let len = nums.len();
    let neg = (diff >> 1) as usize;
    let mut res = vec![vec![0; neg + 1]; len + 1];
    res[0][0] = 1;
    for i in 0..len {
        let current = nums[i] as usize;
        for j in 0..=neg {
            res[i + 1][j] = res[i][j];
            if j >= current {
                res[i + 1][j] += res[i][j - current];
            }
        }
    }
    res[len][neg]
}