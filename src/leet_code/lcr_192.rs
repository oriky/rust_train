use std::collections::HashMap;

pub fn pair_sums(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut dict = HashMap::with_capacity(100000);
    let mut res = Vec::with_capacity(50000);
    for num in nums {
        match dict.get_mut(&(target - num)) {
            None => {
                *dict.entry(num).or_insert(0) += 1;
            }
            Some(i) => {
                res.push(vec![target - num, num]);
                if *i > 1 {
                    *i -= 1;
                } else {
                    dict.remove(&(target - num));
                }
            }
        };
    }
    res
}