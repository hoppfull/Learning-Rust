fn main() {
    let x = 5;
    let y = &x; // y is an immutable variable with an immutable reference to an immutable value
    let z = x; // z is a copy of x

    println!("x={}", x); // 5
    println!("y={}", y); // 5
    println!("z={}", z); // 5

    let x = 42;
    let mut y = &x; // ys is a mutable variable with an immutable reference to an immutable value

    println!("x={}", x); // 42
    println!("y={}", y); // 42
    y = &z; // y now references another immutable value
    println!("y={}", y); // 5

    let mut x = 3.14;
    println!("x={}", x); // 3.14
    {
        let y = &mut x; // y is an immutable variable with a mutable reference to a mutable value
        *y = 42.; // mutate the value y references
        println!("y={}", y); // 42
    }
    println!("x={}", x); // 42
    {
        let y = &x; // y is an immutable variable with an immutable reference to a mutable value
        println!("x={}", x); // 42
        println!("y={}", y); // 42
    }
    /*
    let y: &i32; // uninitiated immutable reference
    {
        let x = 5;
        y = &x; // not allowed
        // This is not allowed because x is freed when it goes
        // out of scope so y would reference nothing
    }
    let x = 5;
    y = &x; // not allowed
    // This is not allowed because resources are freed in opposite order
    // than they were declared so x is going out of scope before y;
    */
}
