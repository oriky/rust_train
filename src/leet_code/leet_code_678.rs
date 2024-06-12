/**
 给你一个只包含三种字符的字符串，支持的字符类型分别是 '('、')' 和 '*'。请你检验这个字符串是否为有效字符串，如果是 有效 字符串返回 true 。

有效 字符串符合如下规则：

任何左括号 '(' 必须有相应的右括号 ')'。
任何右括号 ')' 必须有相应的左括号 '(' 。
左括号 '(' 必须在对应的右括号之前 ')'。
'*' 可以被视为单个右括号 ')' ，或单个左括号 '(' ，或一个空字符串 ""。

思路：贪心
 两次遍历，第一次从左往右，遇到（和*则计数加1，遇到）计数减一，当计数小于0跳出循环；
         第二次从右往左，计数逻辑相同；
两次遍历后，如果从左往右和从右往左的计数都大于等于0，则是一个有效的括号
 */
pub fn check_valid_string(s: String) -> bool {
    let arr: Vec<char> = s.chars().collect();
    let mut left = 0;
    for &c in arr.iter() {
        if left < 0 { break; }
        if c == '(' || c == '*' { left += 1 } else { left -= 1 }
    }
    let mut right = 0;
    for &c in arr.iter().rev() {
        if right < 0 { break; }
        if c == ')' || c == '*' { right += 1 } else { right -= 1 }
    }
    right >= 0 && left >= 0
}