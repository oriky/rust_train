/**
给你一个 非空 整数数组 nums ，除了某个元素只出现一次以外，其余每个元素均出现两次。找出那个只出现了一次的元素。
*/
pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut res = 0;
    for num in nums {
        res ^= num;
    }
    res
}