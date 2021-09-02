
pub fn single_fn(mut n: i32) -> i32 {
    let mut fact = n;
    while n > 1 {
        fact = fact * (n - 1);
        n = n - 1;
    }
    fact
}

pub fn recursive(n: i32) -> i32 {
    if n > 1 {
        n * recursive(n - 1)
    } else {
        1
    }
}