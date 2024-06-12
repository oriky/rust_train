/**
整数数组 sockets 记录了一个袜子礼盒的颜色分布情况，其中 sockets[i] 表示该袜子的颜色编号。礼盒中除了一款撞色搭配的袜子，每种颜色的袜子均有两只。

请设计一个程序，在时间复杂度 O(n)，空间复杂度O(1) 内找到这双撞色搭配袜子的两个颜色编号。
 */
pub fn sock_collocation(sockets: Vec<i32>) -> Vec<i32> {
    let mut res = 0;
    let mut shift = 1;
    for &i in sockets.iter() {
        res ^= i;
    }
    while res & shift == 0 {
        shift <<= 1;
    }
    let mut first = 0;
    let mut second = 0;
    for i in sockets {
        if i & shift == 0 {
            first ^= i;
        } else {
            second ^= i;
        }
    }
    vec![first, second]
}