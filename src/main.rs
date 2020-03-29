use mathtools::algebra::real::Float;

fn main() {
    let x = Float::with_val(4, 1);
    let y = Float::with_val(32, 2);
    let z = Float::with_val(32, x + y);
    println!("{}", z);
}
