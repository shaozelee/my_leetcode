// 每次你可以爬 1 或 2 个台阶。你有多少种不同的方法可以爬到楼顶呢？
/*
f(3) = f(2) + f(1) = 2 + 1 = 3
f(4) = f(3) + f(2) = 3 + 2 = 5
f(5) = f(4) + f(3) = 5 + 3 = 8
*/
pub fn climb_stairs(n: i32) -> i32 {
    // match n {
    //     0 | 1 => 1,
    //     _ => climb_stairs(n - 1) + climb_stairs(n - 2),
    // }
    if n <= 2 {
        return n;
    }
    let (mut prev, mut curr) = (1, 2);
    for _ in 3..=n {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }
    curr
}
