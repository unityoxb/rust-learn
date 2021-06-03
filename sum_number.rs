// 定义求和的方法
fn sum_number(v:&[u32]) -> Option<u32> {
    let mut r:u32 = 0;

    for n in v {
        r = r + n;
    }
    if r > u32::MAX {
        return None;
    }
    Some(r)

}

fn main() {
    let num = &[3, 5, 8, 4143636300, 4143646666];
    let res = sum_number(num);
    println!("计算结果是：{:?}", res);
}
