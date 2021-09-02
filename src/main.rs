use proj_modules::temperature as t;
use proj_modules::fibonacci as fibo;
use proj_modules::factorial as facto;

fn main() {
    let mut farenheit = 98.6;
    let celcius = t::f2c(farenheit);
    println!("Farenheit = {}", farenheit);
    println!("Celcius = {}", celcius);

    farenheit = t::c2f(celcius);

    println!("Celcius = {}", celcius);
    println!("Farenheit = {}", farenheit);

    // Fibonacci 
    let mut i = 0;
    let mut c = 1;
    let n = 5;

    while c <= n {
        println!("{}", fibo::fibo(i));
        i = i + 1;
        c = c + 1;
    }

    // Factorial

    let n = 5;
    let fact = facto::single_fn(n);
    println!("Factorial using while  of {} = {}",n,fact);

    println!("Factorial recursice 0f {} = {}", 6, facto::recursive(6))
}
