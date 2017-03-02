fn main() {
    // Purpose of ownership is to ensure memory safety
    let u = vec![1, 2, 3]; // u owns the value stored in memory

    let v = u; // v now owns the value stored in memory for u

    // println!("u[0]={}", u[0]); // this is not allowed!
    println!("v[0]={}", v[0]); // this is allowed
    f(v); // the xs-parameter now owns the value stored in memory for u
    // f doesn't return the value back so it's destroyed and we have no working references left


    // println!("u[0]={}", u[0]); // this is not allowed!
    // println!("v[0]={}", v[0]); // this is not allowed!

    // memory safety!
}

fn f(xs: Vec<i32>) {
    println!("v[0]={}", xs[0]);
    // the memory stored in xs's location gets destroyed when it goes out of scope
}
