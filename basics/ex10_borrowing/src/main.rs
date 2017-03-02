fn main() {
    let u = vec![1, 2, 3];
    let v = vec![4, 5, 6];

    g(v); // ownership of memory for v is transfered to the xs parameter for function g
    // g doesn't give ownership back so v is useless now
    // println!("v[0]={}", v[0]); // This is not allowed

    f(&u); // ownership of memory for u is borrowed by the parameter xs for function f
    // ownership is returned to u so u can continue to be used!
    println!("v[0]={}", u[0]); // This is allowed

    // h(&mut u); // h takes a mutable reference so this is not allowed

    let mut w = vec![7, 8, 9];
    println!("w[0]={}", w[0]); // This is allowed
    h(&mut w);
    println!("w[0]={}", w[0]); // This is allowed
}

fn f(xs: &Vec<i32>) {
    println!("xs[0]={}", xs[0]);
}

fn g(xs: Vec<i32>) {
    println!("xs[0]={}", xs[0]);
}

fn h(xs: &mut Vec<i32>) {
    xs[0] = 42;
}
