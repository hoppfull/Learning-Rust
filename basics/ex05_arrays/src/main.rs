fn main() {
    let xs = [1, 2, 3, 4, 5]; // initialize fixed size immutable array
    let mut ys = [1, 2, 3]; // initialize fixed size mutable array
    let zs = [1; 20]; // initialize fixed size immutable array of 20 ones
    println!("xs.len()={}", xs.len()); // 5
    println!("ys.len()={}", ys.len()); // 3
    println!("zs.len()={}", zs.len()); // 20

    println!("ys[2]={}", ys[2]); // 3
    ys[2] = 7;
    println!("ys[2]={}", ys[2]); // 7

    // slices:
    let xs_all = &xs[..];
    println!("xs_all.len()={}", xs_all.len()); // 5
    let xs_1to3 = &xs[1..3];
    println!("xs_1to3.len()={}", xs_1to3.len()); // 2
    println!("xs_1to3=[{0}, {1}]", xs_1to3[0], xs_1to3[1]); // [2, 3]
}
