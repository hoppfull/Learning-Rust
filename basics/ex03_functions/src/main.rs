fn main() {
    print_int(add(5, f(7))); // 19
    print_float(9.); // 9
    let g: fn(i32) -> i32 = f; // functions are first class
    print_int(g(12)); // 24
    //diverges();
}

// return type is inferred:
fn print_int(x: i32) {
    println!("{}", x);
}

// function returns i32:
fn add(x: i32, y: i32) -> i32 {
    x + y // return expression should not end with ; I think
}

// function returns unit?
fn print_float(x: f32) -> () {
    println!("{}", x)
}

// allowed but considered poor style:
fn f(x: i32) -> i32 {
    return x * 2; // return should be used only for early returns
}
/*
fn diverges() -> ! {
    panic!("exception!?");
}*/
