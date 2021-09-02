use proj_modules::temperature as t;

fn main() {
    let mut farenheit = 98.6;
    let celcius = t::f2c(farenheit);
    println!("Farenheit = {}", farenheit);
    println!("Celcius = {}", celcius);

    farenheit = t::c2f(celcius);

    println!("Celcius = {}", celcius);
    println!("Farenheit = {}", farenheit);
}
