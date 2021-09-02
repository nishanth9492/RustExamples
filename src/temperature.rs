
pub fn f2c(farenheit : f32) -> f32 {
    const SUB_VALUE : f32 = 32.0;
    const MUL_VALUE : f32 = 5.0 / 9.0;
    let celcius : f32 = (farenheit - SUB_VALUE) * (MUL_VALUE);
    celcius
}

pub fn c2f(celcius : f32) -> f32 {
    const ADD_VALUE : f32 = 32.0;
    const MUL_VALUE : f32 = 9.0 / 5.0;
    let farenheit : f32 = (celcius * MUL_VALUE) + (ADD_VALUE);
    farenheit
}