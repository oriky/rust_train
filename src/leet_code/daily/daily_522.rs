/*
给定字符串列表 strs ，返回其中 最长的特殊序列 的长度。如果最长特殊序列不存在，返回 -1 。

特殊序列 定义如下：该序列为某字符串 独有的子序列（即不能是其他字符串的子序列）。

s 的 子序列可以通过删去字符串 s 中的某些字符实现。

例如，"abc" 是 "aebdc" 的子序列，因为您可以删除"aebdc"中的下划线字符来得到 "abc" 。"aebdc"的子序列还包括"aebdc"、 "aeb" 和 "" (空字符串)。
*/
pub fn find_lu_slength(strs: Vec<String>) -> i32 {
    let mut res = 0;
    'outer:for (i, s) in strs.iter().enumerate() {
        if s.len() <= res { continue; }
        for (j, t) in strs.iter().enumerate() {
            if i != j && is_sub(s.as_bytes(), t) {
                continue 'outer;
            }
        }
        res = s.len();
    }
    if res == 0 { -1 } else { res as i32 }
}

fn is_sub(s: &[u8], t: &str) -> bool {
    let mut i = 0;
    for c in t.bytes(){
        if c == s[i]{
            i += 1;
        }
        if i == s.len() {
            return true;
        }
    }
    false
}