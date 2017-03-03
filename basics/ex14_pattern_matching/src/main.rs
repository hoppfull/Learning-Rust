// https://doc.rust-lang.org/book/patterns.html

enum MyUnion {
    Success { data: String, value: f32 },
    Failure(String),
    Nothing,
}

fn main() {
    let success = MyUnion::Success {
        data: "PIE!".to_string(),
        value: 3.14,
    };
    let failure = MyUnion::Failure("Boo!".to_string());
    let nothing = MyUnion::Nothing;

    match success {
        MyUnion::Success { value, .. } => println!("value={}", value),
        _ => println!("whatevs"),
    }; // 3.14

    f(success);
    f(failure);
    f(nothing);

    let x = 5;
    let s = match x {
        e if e < 0 => "Negative",
        0 => "Zero",
        3 | 4 => "Three or Four",
        5 => "Five",
        6...10 => "Six, Seven, Eight, Nine or Ten",
        _ => "Dunno",
    };
    println!("{}", s); // Five

    let t = (5, 3, "Hello");
    match t {
        (1, 3, "Hello") => println!("one, tree and hi"),
        (5, _, "Boo") => println!("five, whatevs and boo"),
        (_, _, "Hello") => println!("whatevs, whatevs and hi"),
        (..) => println!("whatevs, whatevs and whatevs"),
    };
    let (a, _, b) = t;
    println!("{}", a); // 5
    println!("{}", b); // "Hello"
    println!("{:?}", t); // (5, 3, "Hello")

    g(Some(MyStruct {
        name: "The Answer".to_string(),
        value: 42,
    }));
}

struct MyStruct {
    name: String,
    value: i32,
}

fn f(response: MyUnion) -> () {
    match response {
        MyUnion::Success { data, .. } => println!("Success {{ data={}, value=whatevs }}", data),
        MyUnion::Failure(s) => println!("Failure({})", s),
        MyUnion::Nothing => println!("Nothing"),
    }
}

fn g(option: Option<MyStruct>) {
    match option {
        Some(MyStruct { name, value }) => {
            println!("Some MyStruct = {{ name={}, value={} }}", name, value)
        }
        None => println!("None"),
    };
}
