fn main() {
    let x = true;
    if x {
        println!("x is true!");
    } else {
        println!("x is false!");
    }
    // if is also an expression:
    let y = if !x { 42. } else { 3.14 };
    println!("y={}", y); // 3.14
}
