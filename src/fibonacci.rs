pub fn fibo(n: i32) -> i32{
    if n <= 1 {
        n
    } else {
        fibo(n-1) + fibo(n-2)
    }
}