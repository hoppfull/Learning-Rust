fn main() {
    let mut i = 1;
    // infite loop:
    loop {
        println!("{}", i);
        if i >= 10 {
            break; // break stops the loop
        }
        i += 1;
    }
    while i > 0 {
        println!("{}", i);
        i -= 1;
    }
    for x in 1..10 {
        println!("{}", x);
    }
    let xs = [5; 6];
    for x in xs.iter() {
        println!("{}", x);
    }
    for (i, x) in (3..10).enumerate() {
        println!("(i={},x={})", i, x);
    }
    'myloop: for x in 0..10 {
        if x > 5 {
            break 'myloop;
        }
        println!("{}", x);
    }
}
