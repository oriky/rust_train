/**
给你一个整数数组 nums ，除某个元素仅出现 一次 外，其余每个元素都恰出现 三次 。请你找出并返回那个只出现了一次的元素。

你必须设计并实现线性时间复杂度的算法且使用常数级空间来解决此问题。

1 <= nums.length <= 3 * 10^4
-2^31 <= nums[i] <= 2^31 - 1

思路：除了一个数字出现一次，其他数字都出现三次，如果按照所有数字的二进制累加，
最终的结果再按每一位mod 3,剩余的就是出现一次的二进制数字，由于无法使用数字
将32位数字按十进制存储，所以使用一个长度为32的数组来模拟存储i32类型数字每位二进制
 */
pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut res = 0;
    let mut shift = 1;
    let mut arr = vec![0; 32];
    for num in nums {
        for i in 0..32 {
            arr[i] += if num & shift != 0 { 1 } else { 0 };
            shift <<= 1;
        }
        shift = 1;
    }
    for i in arr {
        if i % 3 != 0 {
            res += shift;
        }
        shift <<= 1;
    }
    res
}
