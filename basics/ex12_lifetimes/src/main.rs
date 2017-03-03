static MYCONST: i32 = 5;

fn main() {
    let mut x = 5;
    let y: &i32;
    {
        let z = 42;
        y = f(&mut x, &z);
    }
    println!("y={}", y); // 89
    let my_struct = MyStruct { my_field: y };
    println!("my_struct.my_field={}", my_struct.my_field); // 89
    let y: &'static i32 = &MYCONST; // 'static lifetime is the lifetime of the entire program
    println!("y={}", y); // 5
}

// Since we don't know from the type signature of f which reference is returned, we "mark" the
// references with lifetime parameters. In this example we show that it is the reference of a that
// is returned:
fn f<'r, 's>(a: &'r mut i32, b: &'s i32) -> &'r i32 {
    *a += *b * 2;
    a
}

// MyStruct needs a lifetime parameter to ensure that any reference to a MyStruct cannot outlive
// the reference of my_field:
struct MyStruct<'r> {
    my_field: &'r i32,
}
