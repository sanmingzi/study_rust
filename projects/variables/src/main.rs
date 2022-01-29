const MAX_SIZE: u32 = 100_000;

fn main() {
    // MAX_SIZE = 100; error
    println!("MAX_SIZE is: {}", MAX_SIZE);

    let x = 100;
    // x = 200; error
    println!("x is: {}", x);
    let x = 200;
    println!("x is: {}", x);
    let x = "Knight";
    println!("x is: {}", x);

    let mut x = 100;
    println!("mut x is: {}", x);
    x = 200;
    println!("mut x is: {}", x);
    // x = "Knight";
}
