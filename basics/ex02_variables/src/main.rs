fn main() {
    let x = 5; // Initializing and assigning a variable
    println!("1st x={}", x); // 5

    let x = 6; // Shadowing (this x replaces previous x)
    println!("2nd x={}", x); // 6

    let x: i32 = 5; // type annotations
    println!("3rd x={}", x); // 5
    // immutability by default, x = 7; would cause compilation error

    let mut x = 7;
    println!("4th x={}", x); // 7

    x = 8;
    println!("4th x={}", x); // 8

    let x: i32; // since x isn't initialized type information is required
    x = 9; // we can't use x before it has been initialized
    println!("5th x={}", x); // 9
    {
        // local scope:
        let x = 0;
        println!("1st local x={}", x); // 0
    }
    println!("5th x={}", x); // 9

    /*
    let mut x = 0;
    x = 1;
    let x = x; // new x is immutable and equals 1
    println!("7th x={}", x); // 1
    */

    let x = "yay!";
    println!("6th x={}", x);
}
